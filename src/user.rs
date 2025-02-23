use crate::auth_middleware::Claims;
use axum::{
    Extension, Json,
    body::Body,
    extract::State,
    http::{Response, StatusCode},
};
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tracing::warn;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AvatarPayload {
    pub avatar_id: i32,
}

#[axum::debug_handler]
pub async fn metadata(
    // Added pub
    State(pool): State<Arc<Pool<Postgres>>>,
    Extension(claims): Extension<Arc<Claims>>,
    Json(payload): Json<AvatarPayload>,
) -> Result<Response<Body>, StatusCode> {
    // Added <Body>
    let response = sqlx::query("UPDATE users SET avatar_id = $1 WHERE username = $2")
        .bind(payload.avatar_id)
        .bind(&claims.sub)
        .execute(&*pool)
        .await;

    match response {
        Ok(_) => Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::empty())
            .unwrap()),
        Err(_) => {
            warn!("Unauthorized Login");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
