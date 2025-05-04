use axum::{Json, extract::State, http::StatusCode};
use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken::{EncodingKey, Header, encode};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{env, sync::Arc};
use tracing::{error, info};

#[derive(Debug, Deserialize)]
pub struct SignInPayload {
    username: String,
    password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SignInResponse {
    token: String,
}

#[derive(sqlx::Type, serde::Serialize, serde::Deserialize, Debug)]
#[sqlx(type_name = "role_enum", rename_all = "lowercase")]
pub enum Role {
    User,
    Admin,
}

#[derive(Deserialize)]
pub struct SignUpPayload {
    username: String,
    email_id: String,
    password: String,
    avatar_id: Option<i32>,
    role: Role,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: i32,
    exp: usize,
    role: String,
}

pub async fn signin(
    State(pool): State<Arc<sqlx::PgPool>>,
    Json(payload): Json<SignInPayload>,
) -> Result<Json<SignInResponse>, StatusCode> {
    let response = sqlx::query!(
        "SELECT id, username, password_hash, role::TEXT FROM users WHERE username = $1",
        payload.username
    )
    .fetch_one(&*pool)
    .await;

    match response {
        Ok(record) => {
            let password_matches =
                bcrypt::verify(&payload.password, &record.password_hash).unwrap_or(false);
            if password_matches {
                static SECRET_KEY: Lazy<Vec<u8>> = Lazy::new(|| {
                    dotenv().ok();
                    env::var("SECRET_KEY_JWT")
                        .expect("Error in getting secret key")
                        .into_bytes()
                });

                let expiration = Utc::now() + Duration::hours(24);
                let claims = Claims {
                    sub: record.id,
                    exp: expiration.timestamp() as usize,
                    role: record.role.unwrap_or_else(|| "user".to_string()),
                };
                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(&*SECRET_KEY),
                )
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                let _ = sqlx::query!(
                    "UPDATE users SET last_login = CURRENT_TIMESTAMP WHERE id = $1",
                    record.id
                )
                .execute(&*pool)
                .await;

                Ok(Json(SignInResponse { token }))
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        }
        Err(err) => {
            error!("Database error during signin: {:?}", err);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

pub async fn signup(
    State(pool): State<Arc<sqlx::PgPool>>,
    Json(payload): Json<SignUpPayload>,
) -> Result<StatusCode, StatusCode> {
    info!("User attempting to sign in: {}", payload.username);

    let password_hash = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let role_str = match payload.role {
        Role::User => "User",
        Role::Admin => "Admin",
    };
    info!(
        "Inserting user: username={}, email={} password={}, avatar_id={:?}, role={}",
        payload.username, payload.email_id, payload.password, payload.avatar_id, role_str
    );

    let response = sqlx::query(
        "INSERT INTO users (username, email,  password_hash, avatar_id, role) VALUES ($1, $2, $3, $4, $5::role_enum)",
    )
    .bind(&payload.username)
    .bind(&payload.email_id)
    .bind(password_hash)
    .bind(payload.avatar_id)
    .bind(role_str)
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
