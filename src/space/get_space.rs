use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow, Row, postgres::PgRow};
use std::sync::Arc;
use tracing::error;

#[derive(Deserialize)]
pub struct GetSpacePayload {
    space_id: i32,
}

#[derive(Serialize, FromRow)]
pub struct GetSpaceResponse {
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

pub async fn get_space(
    State(pool): State<Arc<sqlx::PgPool>>,
    Json(payload): Json<GetSpacePayload>,
) -> Result<Json<GetSpaceResponse>, StatusCode> {
    let response = sqlx::query_as::<_, GetSpaceResponse>("SELECT (map_id, name, description, width , height, background_url, thumbnail_url, max_occupancy, is_private, default_spawn_x, default_spawn_y) FROM spaces WHERE space_id = $1 ")
        .bind(payload.space_id)
        .fetch_one(&*pool)
        .await;

    match response {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            error!("Error getting space {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}
