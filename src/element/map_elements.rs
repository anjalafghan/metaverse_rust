use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::error;

#[derive(Serialize, Deserialize)]
pub struct CreateMapElementsPayload {
    map_id: i32,
    template_id: i32,
    x: i32,
    y: i32,
    z_index: i32,
    target_space_id: i32,
    custom_properties: serde_json::Value,
}

pub async fn create_map_elements(
    State(pool): State<Arc<sqlx::PgPool>>,
    Json(payload): Json<CreateMapElementsPayload>,
) -> Result<StatusCode, StatusCode> {
    let response = sqlx::query("INSERT INTO map_elements (map_id, template_id, x, y, z_index, target_space_id, custom_properties) VALUES ($1, $2, $3, $4, $5, $6, $7)")
 .bind(payload.map_id)
 .bind(payload.template_id)
.bind(payload.x)
.bind(payload.y)
.bind(payload.z_index)
.bind(payload.target_space_id)
.bind(payload.custom_properties)
.execute(&*pool)
.await;

    match response {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            error!("Error creating map elements {e}");
            Err(StatusCode::FORBIDDEN)
        }
    }
}
