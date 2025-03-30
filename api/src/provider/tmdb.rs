use crate::model::content::{ContentInput, ContentType};
use crate::provider::{Provider, ProviderKey};
use chrono::NaiveDate;
use reqwest::header;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::str::FromStr;
use tokio::sync::{RwLock, RwLockReadGuard};

const BASE: &str = "https://api.themoviedb.org/3";

pub struct TmdbProvider {
    client: reqwest::Client,
    genres: RwLock<BTreeMap<usize, String>>,
}

impl TmdbProvider {
    pub fn new(tmdb_token: &str) -> Option<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {tmdb_token}").parse().ok()?,
        );

        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .ok()?;

        Some(Self {
            client,
            genres: Default::default(),
        })
    }

    async fn update_genres_for(&self, content_type: ContentType) -> Result<(), reqwest::Error> {
        let content_type = match content_type {
            ContentType::Movie => "movie",
            ContentType::Show => "tv",
        };

        let res = self
            .client
            .get(format!("{BASE}/genre/{content_type}/list"))
            .query(&[("language", "fr")])
            .send()
            .await?
            .json::<GenresResponse>()
            .await?;

        let mut genres = self.genres.write().await;

        for g in res.genres {
            genres.insert(g.id, g.name);
        }

        Ok(())
    }
}

impl Provider for TmdbProvider {
    async fn search(&self, query: &str) -> Result<Vec<ContentInput>, Box<dyn std::error::Error>> {
        let res = self
            .client
            .get(format!("{BASE}/search/multi"))
            .query(&[("query", query), ("language", "fr-FR")])
            .send()
            .await?
            .json::<Response>()
            .await?;

        let should_update = {
            let genres = self.genres.read().await;
            res.results
                .iter()
                .filter_map(|e| e.genre_ids.clone())
                .flatten()
                .any(|e| !genres.contains_key(&e))
        };

        if should_update {
            self.update_genres_for(ContentType::Show).await?;
            self.update_genres_for(ContentType::Movie).await?;
        }

        let genres = self.genres.read().await;
        res.results
            .into_iter()
            .filter_map(|r| r.into_content(&genres))
            .collect::<Result<_, _>>()
            .map_err(Into::into)
    }
}

#[derive(Deserialize)]
struct GenresResponse {
    genres: Vec<GenresResponseGenre>,
}

#[derive(Deserialize)]
struct GenresResponseGenre {
    id: usize,
    name: String,
}

#[derive(Deserialize)]
struct Response {
    results: Vec<ResponseResult>,
}

#[derive(Deserialize)]
struct ResponseResult {
    id: usize,
    #[serde(alias = "name")]
    title: String,
    overview: Option<String>,
    poster_path: Option<String>,
    media_type: String,
    genre_ids: Option<Vec<usize>>,
    #[serde(alias = "first_air_date")]
    release_date: Option<String>,
    vote_average: Option<f64>,
    vote_count: Option<i32>,
    backdrop_path: Option<String>,
}

impl ResponseResult {
    fn into_content(
        self,
        genres: &RwLockReadGuard<BTreeMap<usize, String>>,
    ) -> Option<Result<ContentInput, &'static str>> {
        let content_type = match self.media_type.as_str() {
            "movie" => ContentType::Movie,
            "tv" => ContentType::Show,
            _ => return None,
        };

        let Some(overview) = self.overview else {
            return Some(Err("ResponseResult should have an overview"));
        };
        let Some(release_date) = self.release_date else {
            return Some(Err("ResponseResult should have a release date"));
        };
        let Some(genre_ids) = self.genre_ids else {
            return Some(Err("ResponseResult should have genre ids"));
        };

        let content = ContentInput {
            provider_id: ProviderKey::TMDB(self.id.to_string()),
            content_type,
            title: self.title,
            overview,
            poster: self
                .poster_path
                .map(|p| format!("https://image.tmdb.org/t/p/w500{p}")),
            release_date: NaiveDate::from_str(&release_date).ok(),
            genres: genre_ids
                .iter()
                .flat_map(|id| {
                    genres
                        .get(id)
                        .map(|e| e.split(" & ").collect::<Vec<_>>())
                        .unwrap_or_default()
                })
                .map(ToString::to_string)
                .collect(),
            backdrop: self.backdrop_path.map(|p| format!("https://image.tmdb.org/t/p/w1280{p}")),
            vote_average: self.vote_average,
            vote_count: self.vote_count,
        };

        Some(Ok(content))
    }
}
