use crate::model::content::ContentType;
use chrono::NaiveDateTime;
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Clone, sqlx::FromRow)]
pub struct Recommendation {
    pub content_id: Uuid,
    pub score: f64,
    pub method: Vec<String>,
    pub o1: i64,
    pub o2: i64,
}

#[derive(Debug)]
pub struct RecommendationView {
    pub content_id: Uuid,
    pub score: f64,
    pub method: Vec<String>,
}

impl From<Recommendation> for RecommendationView {
    fn from(recommendation: Recommendation) -> Self {
        Self {
            content_id: recommendation.content_id,
            score: recommendation.score,
            method: recommendation.method,
        }
    }
}

pub struct RecommendationParameters {
    pub recommendation_id: Uuid,
    pub users_input: Vec<Uuid>,
    pub not_seen_by: Vec<Uuid>,
    pub disable_content_type_filter: bool,
    pub content_type: ContentType,
    pub disable_genre_filter: bool,
    pub genres: Vec<String>,
}

#[derive(Clone)]
pub struct RecommendationParametersInput {
    pub users_input: Vec<Uuid>,
    pub not_seen_by: Vec<Uuid>,
    pub disable_content_type_filter: bool,
    pub content_type: ContentType,
    pub disable_genre_filter: bool,
    pub genres: Vec<String>,
}

impl RecommendationParametersInput {
    pub fn hash(&self) -> [u8; 32] {
        let mut cloned = self.clone();
        cloned.users_input.sort();
        cloned.not_seen_by.sort();
        cloned.genres.sort();
        let mut hasher = Sha256::new();
        hasher.update(
            cloned
                .users_input
                .into_iter()
                .map(|e| e.as_u128().to_string())
                .collect::<Vec<_>>()
                .join("|")
                .as_bytes(),
        );
        hasher.update(
            cloned
                .not_seen_by
                .into_iter()
                .map(|e| e.as_u128().to_string())
                .collect::<Vec<_>>()
                .join("|")
                .as_bytes(),
        );
        hasher.update([u8::from(cloned.disable_content_type_filter)]);
        hasher.update(cloned.content_type.to_string().as_bytes());
        hasher.update([u8::from(cloned.disable_genre_filter)]);
        hasher.update(cloned.genres.join("|").as_bytes());

        hasher.finalize().as_slice().try_into().unwrap()
    }

    pub fn hydrate(self, recommendation_id: Uuid) -> RecommendationParameters {
        RecommendationParameters {
            recommendation_id,
            users_input: self.users_input,
            not_seen_by: self.not_seen_by,
            disable_content_type_filter: self.disable_content_type_filter,
            content_type: self.content_type,
            disable_genre_filter: self.disable_genre_filter,
            genres: self.genres,
        }
    }
}

pub struct RecommendationDb {
    pub recommendation_id: Uuid,
    pub hash: Vec<u8>,
    pub updated_at: NaiveDateTime,
    pub refcount: i64,
}
