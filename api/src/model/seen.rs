use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::content::ContentView;

#[derive(Serialize, Clone, Debug)]
pub struct SeenContent {
    pub content: ContentView,
    pub grade: Option<f64>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SeenContentInput {
    pub content_id: Uuid,
    pub grade: Option<f64>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SeenGradeInput {
    pub content_id: Uuid,
    pub grade: Option<f64>,
}
