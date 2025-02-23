use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, info, warn};

#[derive(Debug, Deserialize)]
pub struct SignInPayload {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct SignUpPayload {
    username: String,
    password: String,
    avatar_id: i32,
    role: String,
}

pub async fn signin(
    State(pool): State<Arc<sqlx::PgPool>>,
    Json(payload): Json<SignInPayload>,
) -> Result<Json<String>, StatusCode> {
    let response = sqlx::query!(
        "SELECT username, password FROM users WHERE username=$1",
        payload.username
    )
    .fetch_one(&*pool)
    .await;

    match response {
        Ok(record) => {
            if record.username == payload.username && record.password == payload.password {
                Ok(Json("1011".to_string()))
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        }
        Err(err) => {
            error!("Database error during signin: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn signup(
    State(pool): State<Arc<sqlx::PgPool>>,
    Json(payload): Json<SignUpPayload>,
) -> Result<StatusCode, StatusCode> {
    info!("User attempting to sign in: {}", payload.username);
    let response = sqlx::query!(
        "INSERT INTO users (username, password, avatar_id, role) VALUES ($1, $2, $3, $4)",
        payload.username,
        payload.password,
        payload.avatar_id,
        payload.role
    )
    .execute(&*pool)
    .await;

    match response {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(error) => {
            if error.to_string().contains("duplicate") {
                Ok(StatusCode::CONFLICT)
            } else {
                error!("Database error during signin: {:?}", error);
                Ok(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}
