use crate::provider::ProviderKey;
use chrono::{NaiveDate, NaiveDateTime, Utc};
use serde::Serialize;
use std::str::FromStr;
use std::time::Duration;
use uuid::Uuid;

#[derive(sqlx::Type, Debug, Serialize, Copy, Clone, PartialEq, Eq)]
#[sqlx(type_name = "content_type", rename_all = "lowercase")]
pub enum ContentType {
    Movie,
    Show,
}

pub struct Content {
    pub content_id: Uuid,
    pub provider_id: String,
    pub updated_at: NaiveDateTime,
    pub content_type: ContentType,
    pub title: String,
    pub overview: String,
    pub poster: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub genres: Vec<String>,
    // pub backdrop: Option<String>,
    // pub vote_average: f32,
    // pub vote_count: usize
}

impl Content {
    pub fn is_expired(&self, ttl: Duration) -> bool {
        self.updated_at + ttl > Utc::now().naive_utc()
    }

    pub fn provider_info(&self) -> Result<ProviderKey, &str> {
        ProviderKey::from_str(&self.provider_id)
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct ContentView {
    pub content_id: Uuid,
    pub content_type: ContentType,
    pub title: String,
    pub overview: String,
    pub poster: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub genres: Vec<String>,
    // pub backdrop: Option<String>
    // pub vote_average: f32,
}

impl From<Content> for ContentView {
    fn from(value: Content) -> Self {
        Self {
            content_id: value.content_id,
            content_type: value.content_type,
            title: value.title,
            overview: value.overview,
            poster: value.poster,
            release_date: value.release_date,
            genres: value.genres,
        }
    }
}

pub struct ContentInput {
    pub provider_id: ProviderKey,
    pub content_type: ContentType,
    pub title: String,
    pub overview: String,
    pub poster: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub genres: Vec<String>,
}

impl ContentInput {
    pub fn hydrate(self, content_id: Uuid, updated_at: NaiveDateTime) -> Content {
        Content {
            content_id,
            provider_id: self.provider_id.to_string(),
            updated_at,
            content_type: self.content_type,
            title: self.title,
            overview: self.overview,
            poster: self.poster,
            release_date: self.release_date,
            genres: self.genres,
        }
    }
}

impl PartialEq<Content> for ContentInput {
    fn eq(&self, other: &Content) -> bool {
        self.provider_id.to_string().eq(&other.provider_id)
            && self.content_type.eq(&other.content_type)
            && self.title.eq(&other.title)
            && self.overview.eq(&other.overview)
            && self.poster.eq(&other.poster)
            && self.poster.eq(&other.poster)
            && self.release_date.eq(&other.release_date)
            && self.genres.eq(&other.genres)
    }
}
