use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::error;

#[derive(Serialize, Deserialize)]
pub struct CreateSpaceElementsPayload {
    space_id: i32,
    template_id: i32,
    x: i32,
    y: i32,
    z_index: i32,
    rotation: i32,
    custom_properties: serde_json::Value,
}

pub async fn create_space_elements(
    State(pool): State<Arc<sqlx::PgPool>>,
    Json(payload): Json<CreateSpaceElementsPayload>,
) -> Result<StatusCode, StatusCode> {
    let response = sqlx::query("INSERT INTO space_elements (space_id, template_id, x, y, z_index, rotation, custom_properties) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
 .bind(payload.space_id)
 .bind(payload.template_id)
.bind(payload.x)
.bind(payload.y)
.bind(payload.z_index)
.bind(payload.rotation)
.bind(payload.custom_properties)
.execute(&*pool)
.await;

    match response {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            error!("Error creating space elements {e}");
            Err(StatusCode::FORBIDDEN)
        }
    }
}
