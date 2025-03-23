use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::error;

#[derive(Deserialize)]
pub struct CreateSpacePayload {
    map_id: i32,
    name: String,
    description: String,
    width: i32,
    height: i32,
    background_url: String,
    thumbnail_url: String,
    max_occupancy: i32,
    is_private: bool,
    default_spawn_x: i32,
    default_spawn_y: i32,
}

pub async fn create_space(
    State(pool): State<Arc<sqlx::PgPool>>,
    Json(payload): Json<CreateSpacePayload>,
) -> Result<StatusCode, StatusCode> {
    let response = sqlx::query("INSERT INTO spaces (map_id, name, description, width, height, background_url, thumbnail_url, max_occupancy, is_private, default_spawn_x, default_spawn_y) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)")
        .bind(payload.map_id)
        .bind(payload.name)
        .bind(payload.description)
        .bind(payload.width)
        .bind(payload.height)
        .bind(payload.background_url)
        .bind(payload.thumbnail_url)
        .bind(payload.max_occupancy)
        .bind(payload.is_private)
        .bind(payload.default_spawn_x)
        .bind(payload.default_spawn_y)
        .execute(&*pool)
        .await;
    match response {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            error!("Error creating spaces {e}");
            Err(StatusCode::FORBIDDEN)
        }
    }
}
