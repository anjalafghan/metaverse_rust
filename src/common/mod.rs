use axum::{Json, extract::State, http::StatusCode};
use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken::{EncodingKey, Header, encode};
use once_cell::sync::Lazy;
use serde::Deserialize;
use sqlx::Row;
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
    password: String,
    avatar_id: Option<i32>,
    role: Role,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn signin(
    State(pool): State<Arc<sqlx::PgPool>>,
    Json(payload): Json<SignInPayload>,
) -> Result<Json<SignInResponse>, StatusCode> {
    let response = sqlx::query!(
        "SELECT id, username, password FROM users WHERE username = $1",
        payload.username
    )
    .fetch_one(&*pool)
    .await;

    match response {
        Ok(record) => {
            if record.username == payload.username && record.password == payload.password {
                static SECRET_KEY: Lazy<Vec<u8>> = Lazy::new(|| {
                    dotenv().ok();
                    env::var("SECRET_KEY_JWT")
                        .expect("Error in getting secret key")
                        .into_bytes()
                });

                let expiration = Utc::now() + Duration::hours(24);
                let claims = Claims {
                    sub: record.id.to_string(),
                    exp: expiration.timestamp() as usize,
                };
                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(&*SECRET_KEY),
                )
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                Ok(Json(SignInResponse { token }))
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
    let role_str = match payload.role {
        Role::User => "User",
        Role::Admin => "Admin",
    };
    info!(
        "Inserting user: username={}, password={}, avatar_id={:?}, role={}",
        payload.username, payload.password, payload.avatar_id, role_str
    );

    let response = sqlx::query(
        "INSERT INTO users (username, password, avatar_id, role) VALUES ($1, $2, $3, $4::role_enum)",
    )
    .bind(&payload.username)
    .bind(&payload.password)
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

#[derive(serde::Deserialize, serde::Serialize)]
struct AvatarPayload {
    id: i32,
    name: String,
    image_url: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AvatarResponseBody {
    avatars: Vec<AvatarPayload>,
}

pub async fn get_avatars(
    State(pool): State<Arc<sqlx::PgPool>>,
) -> Result<Json<AvatarResponseBody>, StatusCode> {
    let response = sqlx::query("SELECT id, name, image_url FROM avatars")
        .fetch_all(&*pool)
        .await;

    match response {
        Ok(rows) => {
            let avatars: Vec<AvatarPayload> = rows
                .into_iter()
                .map(|row| AvatarPayload {
                    id: row.get("id"),
                    name: row.get("name"),
                    image_url: row.get("image_url"),
                })
                .collect();

            Ok(Json(AvatarResponseBody { avatars: avatars }))
        }
        Err(e) => {
            error!("Something went wrong {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}
