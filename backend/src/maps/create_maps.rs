use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::error;

#[derive(Serialize, Deserialize)]
pub struct CreateMapPayload {
    world_id: i32,
    name: String,
    width: i32,
    height: i32,
    background_url: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateMapResponse {
    map_id: i32,
}
pub async fn create_map(
    State(pool): State<Arc<sqlx::PgPool>>,
    Json(payload): Json<CreateMapPayload>,
) -> Result<Json<CreateMapResponse>, StatusCode> {
    let result = sqlx::query_scalar!(
        "INSERT INTO maps (world_id, name, width, height, background_url) VALUES ($1, $2, $3, $4, $5) RETURNING id",
        payload.world_id,
        payload.name,
        payload.width,
        payload.height,
        payload.background_url
    )
    .fetch_one(&*pool)
    .await;

    match result {
        Ok(map_id) => Ok(Json(CreateMapResponse { map_id })),
        Err(e) => {
            error!("Error creating space: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
