use crate::api::ApiHandlerState;
use crate::api::errors::ApiError;
use crate::model::content::ContentView;
use crate::provider::Provider;
use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router};
use std::collections::HashMap;

pub async fn search(
    State(api_handler_state): State<ApiHandlerState>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<Json<Vec<ContentView>>, ApiError> {
    let query = query
        .get("query")
        .ok_or(ApiError::bad_request("No `query` parameter".into()))?;

    let search_results = api_handler_state.provider.search(query).await?;

    let mut res = Vec::with_capacity(search_results.len());
    for search_result in search_results {
        let Some(db_content) = api_handler_state
            .db
            .get_content_by_provider_key(&search_result.provider_id)
            .await?
        else {
            let (content_id, updated_at) =
                api_handler_state.db.insert_content(&search_result).await?;

            res.push(search_result.hydrate(content_id, updated_at).into());
            continue;
        };

        let updated_at = if search_result != db_content {
            let (_, updated_at) = api_handler_state.db.update_content(&search_result).await?;

            updated_at
        } else {
            db_content.updated_at
        };

        res.push(
            search_result
                .hydrate(db_content.content_id, updated_at)
                .into(),
        );
    }

    Ok(Json(res))
}

pub fn search_router(api_handler_state: ApiHandlerState) -> Router {
    Router::new()
        .route("/search", get(search))
        .with_state(api_handler_state)
}
