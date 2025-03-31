use crate::api::errors::ApiError;
use crate::api::{ApiHandlerState, AuthContext};
use crate::model::content::ContentView;
use crate::model::recommendation::RecommendationParametersInput;
use axum::extract::State;
use axum::routing::post;
use axum::{Extension, Json, Router};
use tracing::error;

async fn get_recommendations(
    State(api_handler_state): State<ApiHandlerState>,
    Extension(auth_context): Extension<AuthContext>,
    Json(mut input): Json<RecommendationParametersInput>,
) -> Result<Json<Vec<ContentView>>, ApiError> {
    if !input.users_input.contains(&auth_context.user) {
        input.users_input.push(auth_context.user.clone());
    }
    if !input.not_seen_by.contains(&auth_context.user) {
        input.not_seen_by.push(auth_context.user.clone());
    }

    let recommender = api_handler_state
        .recommender
        .get_user_recommendation(input)
        .await
        .map_err(|error| {
            error!(?error);
            ApiError::internal("get_user_recommendation")
        })?;

    let mut res = vec![];
    for _ in 0..100.min(recommender.len().await) {
        if let Ok(Some(r)) = recommender.next().await {
            if let Some(content) = api_handler_state
                .db
                .get_content_by_id(&r.content_id)
                .await?
            {
                res.push(content.into());
            }
        }
    }

    Ok(Json(res))
}

pub fn recommendation_router(api_handler_state: ApiHandlerState) -> Router {
    Router::new()
        .route("/", post(get_recommendations))
        .with_state(api_handler_state)
}
