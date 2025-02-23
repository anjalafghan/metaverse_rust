use axum::{Json, extract::State, http::StatusCode};
use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken::{EncodingKey, Header, encode};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};
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

#[derive(serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
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
                static SECRET_KEY: Lazy<&'static [u8]> = Lazy::new(|| {
                    dotenv().ok();
                    Box::leak(
                        env::var("SECRET_KEY_JWT")
                            .expect("Error in getting secret key")
                            .into_bytes()
                            .into_boxed_slice(),
                    )
                });

                let expiration = Utc::now() + Duration::hours(24);
                let claims = Claims {
                    sub: record.username,
                    exp: expiration.timestamp() as usize,
                };
                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(*SECRET_KEY),
                )
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                Ok(Json(token))
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
