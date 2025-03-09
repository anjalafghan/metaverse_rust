use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::{self, Row, postgres::PgRow};
use std::sync::Arc;
use tracing::error;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateSpaceResponse {
    space_id: i32,
}

#[derive(serde::Deserialize)]
pub struct CreateSpacePayload {
    pub name: String,
    pub width: i32,
    pub height: Option<i32>,
    pub map_id: Option<i32>,
}
pub async fn create_space(
    State(pool): State<Arc<sqlx::PgPool>>,
    Json(payload): Json<CreateSpacePayload>,
) -> Result<Json<CreateSpaceResponse>, StatusCode> {
    let result = sqlx::query_scalar!(
        "INSERT INTO space (name, width, height, map_id) VALUES ($1, $2, $3, $4) RETURNING id",
        payload.name,
        payload.width,
        payload.height,
        payload.map_id
    )
    .fetch_one(&*pool)
    .await;

    match result {
        Ok(space_id) => Ok(Json(CreateSpaceResponse { space_id })),
        Err(e) => {
            error!("Error creating space: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GetAllSpaceResponse {
    spaces: Vec<GetSpace>,
}

#[derive(Serialize, Deserialize)]
struct GetSpace {
    id: i32,
    name: String,
    width: i32,
    height: i32,
    thumbnail: String,
}

pub async fn get_all_space(
    State(pool): State<Arc<sqlx::PgPool>>,
) -> Result<GetAllSpaceResponse, StatusCode> {
    let response = sqlx::query("SELECT id, name, width, height, thumbnail FROM spaces")
        .fetch_all(&*pool)
        .await;

    match response {
        Ok(rows) => {
            let spaces: Vec<GetSpace> = rows
                .into_iter()
                .map(|row| GetSpace {
                    id: row.get("id"),
                    name: row.get("name"),
                    width: row.get("width"),
                    height: row.get("height"),
                    thumbnail: row.get("thumbnail"),
                })
                .collect();
            Ok(GetAllSpaceResponse { spaces })
        }
        Err(e) => {
            error!("Error in getting all spaces: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_space(
    State(pool): State<Arc<sqlx::PgPool>>,
    space_id: i32,
) -> Result<StatusCode, StatusCode> {
    let response = sqlx::query("DELETE FROM spaces WHERE space_id is $1")
        .bind(space_id)
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

#[derive(Serialize, Deserialize)]
struct ActualElement {
    id: String,
    image_url: String,
    is_static: bool,
    height: i32,
    width: i32,
}

#[derive(Serialize, Deserialize)]
struct Elements {
    id: i32,
    x_cor: i32,
    y_cor: i32,
    element: ActualElement,
}

#[derive(Serialize, Deserialize)]
struct GetASpecificSpacePayload {
    width: i32,
    height: i32,
    elements: Vec<Elements>,
}

pub async fn get_a_specific_space(
    State(pool): State<Arc<sqlx::PgPool>>,
    space_id: i32,
) -> Result<GetASpecificSpacePayload, StatusCode> {
    todo!()
}
