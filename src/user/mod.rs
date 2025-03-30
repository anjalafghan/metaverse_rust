use crate::auth_middleware::Claims;
use axum::{
    Extension, Json,
    body::Body,
    extract::State,
    http::{Response, StatusCode},
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Row};
use std::sync::Arc;
use tracing::error;
use tracing::warn;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdateAvatarPayload {
    pub avatar_id: i32,
}

#[axum::debug_handler]
pub async fn metadata(
    // Added pub
    State(pool): State<Arc<Pool<Postgres>>>,
    Extension(claims): Extension<Arc<Claims>>,
    Json(payload): Json<UpdateAvatarPayload>,
) -> Result<Response<Body>, StatusCode> {
    // Added <Body>
    let response = sqlx::query("UPDATE users SET avatar_id = $1 WHERE id = $2")
        .bind(payload.avatar_id)
        .bind(&claims.sub.parse::<i32>().unwrap())
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

#[derive(serde::Deserialize)]
pub struct CreateAvatarPayload {
    name: String,
    image_url: String,
}

pub async fn create_avatar(
    State(pool): State<Arc<sqlx::PgPool>>,
    Json(payload): Json<CreateAvatarPayload>,
) -> Result<StatusCode, StatusCode> {
    let response = sqlx::query("INSERT INTO avatars (name, image_url ) VALUES ($1, $2)")
        .bind(&payload.name)
        .bind(&payload.image_url)
        .execute(&*pool)
        .await;
    match response {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            error!("Avatar could not be created, {}", e);
            Err(StatusCode::FORBIDDEN)
        }
    }
}

// pub async fn get_avatars(
//     State(pool): State<Arc<sqlx::PgPool>>,
//     Extension(claims): Extension<Arc<Claims>>,
// ) -> Result<Json<AvatarPayload>, StatusCode> {
//     let user_id = claims
//         .sub
//         .parse::<i32>()
//         .map_err(|_| StatusCode::BAD_REQUEST)?;
//     let response = sqlx::query_as::<_, AvatarPayload>(
//         "SELECT id, username, avatar_id FROM users WHERE id = $1",
//     )
//     .bind(user_id)
//     .fetch_one(&*pool)
//     .await;
//     match response {
//         Ok(response) => Ok(Json(response)),
//         Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
//         Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
//     }
// }

#[derive(serde::Deserialize, serde::Serialize, sqlx::FromRow)]
struct AvatarPayload {
    id: i32,
    name: String,
    image_url: Option<String>,
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

            Ok(Json(AvatarResponseBody { avatars }))
        }
        Err(e) => {
            error!("Something went wrong {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

#[derive(Deserialize)]
pub struct GetUserMetadataRequestPayload {
    ids: Vec<i32>,
}

#[derive(Serialize, sqlx::FromRow)]
struct UserMetaDataResponsePayload {
    id: i32,
    image_url: Option<String>,
}

#[derive(Serialize)]
pub struct GetUserMetadataResponse {
    avatars: Vec<UserMetaDataResponsePayload>,
}

pub async fn get_metadata_bulk(
    State(pool): State<Arc<sqlx::PgPool>>,
    Json(payload): Json<GetUserMetadataRequestPayload>,
) -> Result<Json<GetUserMetadataResponse>, StatusCode> {
    if payload.ids.is_empty() {
        error!("Payload cannot be empty!");
        return Err(StatusCode::BAD_REQUEST);
    }
    let query = format!(
        "SELECT u.id, a.image_url FROM users u LEFT JOIN avatars a ON u.avatar_id = a.id WHERE u.id = ANY($1) ORDER BY u.id"
    );

    let metadata = sqlx::query_as::<_, UserMetaDataResponsePayload>(&query)
        .bind(&payload.ids)
        .fetch_all(&*pool)
        .await
        .map_err(|e| {
            error!("Database error {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(GetUserMetadataResponse { avatars: metadata }))
}
