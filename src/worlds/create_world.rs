use axum::extract::Extension;
use axum::{Json, extract::State, http::StatusCode};
use std::sync::Arc;
use tracing::error;

use crate::admin_middleware::Claims;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CreateWorldPayload {
    name: String,
    description: String,
    thumbnail_url: String,
    is_public: bool,
}

pub async fn create_world(
    State(pool): State<Arc<sqlx::PgPool>>,
    Extension(claims): Extension<Arc<Claims>>,
    Json(payload): Json<CreateWorldPayload>,
) -> Result<StatusCode, StatusCode> {
    let creator_id = claims.sub.parse::<i32>().map_err(|e| {
        error!("Error getting user id: {}", e);
        StatusCode::FORBIDDEN
    })?;
    let response = sqlx::query("INSERT INTO worlds (name, description, thumbnail_url, creator_id, is_public) VALUES ($1,$2,$3,$4,$5)")
        .bind(payload.name)
        .bind(payload.description)
        .bind(payload.thumbnail_url)
        .bind(creator_id)
        .bind(payload.is_public)
        .execute(&*pool)
        .await;

    match response {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            error!("Error faced while creating world {}", e);
            Err(StatusCode::FORBIDDEN)
        }
    }
}
