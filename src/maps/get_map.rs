use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use std::sync::Arc;
use tracing::error;

#[derive(Serialize, Deserialize, FromRow)]
pub struct GetMapResponse {
    map_id: i32,
    world_id: i32,
    name: String,
    width: i32,
    height: i32,
    background_url: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetMapPayload {
    map_id: i32,
}
pub async fn get_map(
    State(pool): State<Arc<sqlx::PgPool>>,
    Json(payload): Json<GetMapPayload>,
) -> Result<Json<GetMapResponse>, StatusCode> {
    let result = sqlx::query_as::<_, GetMapResponse>(
        "SELECT (id, world_id, name, width, height, background_url) FROM maps WHERE id = $1",
    )
    .bind(payload.map_id)
    .fetch_one(&*pool)
    .await;

    match result {
        Ok(result) => Ok(Json(result)),
        Err(e) => {
            error!("Error getting space: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
