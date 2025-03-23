use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use std::sync::Arc;
use tracing::error;

#[derive(Deserialize)]
pub struct DeleteSpacePayload {
    space_id: i32,
}

pub async fn delete_space(
    State(pool): State<Arc<sqlx::PgPool>>,
    Json(payload): Json<DeleteSpacePayload>,
) -> Result<StatusCode, StatusCode> {
    let response = sqlx::query("DELETE FROM spaces WHERE space_id is $1")
        .bind(payload.space_id)
        .execute(&*pool)
        .await;
    match response {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            error!("Something went wrong {}", e);
            Ok(StatusCode::BAD_REQUEST)
        }
    }
}
