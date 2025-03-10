use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::{self, PgPool, Row, postgres::PgRow, query, query_as};
use std::sync::Arc;
use tracing::error;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CreateElementPayload {
    image_url: String,
    width: i32,
    height: i32,
    is_static: bool,
}

pub async fn create_element(
    State(pool): State<Arc<sqlx::PgPool>>,
    Json(payload): Json<CreateElementPayload>,
) -> Result<StatusCode, StatusCode> {
    let response = sqlx::query(
        "INSERT INTO elements (image_url, width, height, is_static) VALUES ($1, $2, $3, $4)",
    )
    .bind(payload.image_url)
    .bind(payload.width)
    .bind(payload.height)
    .bind(payload.is_static)
    .execute(&*pool)
    .await;

    match response {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            error!("Could not create element {e}");
            Err(StatusCode::FORBIDDEN)
        }
    }
}

pub struct UpdateElementPayload {
    image_url: String,
    element_id: i32,
}

pub async fn update_element(
    State(pool): State<Arc<sqlx::PgPool>>,
    Json(payload): Json<UpdateElementPayload>,
) -> Result<StatusCode, StatusCode> {
    let response = sqlx::query("UPDATE elements SET image_url = $1 WHERE element_id=$2")
        .bind(payload.image_url)
        .bind(payload.element_id)
        .execute(&*pool)
        .await;

    match response {
        Ok(_) => Ok(StatusCode::ACCEPTED),
        Err(e) => {
            error!("Error updating element {e}");
            Err(StatusCode::FORBIDDEN)
        }
    }
}

pub struct AddElementPayload {
    element_id: String,
    space_id: i32,
    x_cor: i32,
    y_cor: i32,
}

pub async fn add_element(
    State(pool): State<Arc<sqlx::PgPool>>,
    Json(payload): Json<AddElementPayload>,
) -> Result<StatusCode, StatusCode> {
    let response =
        sqlx::query("INSERT INTO elements (element_id, space_id, x, y) VALUES ($1, $2, $3, $4)")
            .bind(payload.element_id)
            .bind(payload.space_id)
            .bind(payload.x_cor)
            .bind(payload.y_cor)
            .execute(&*pool)
            .await;

    match response {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            error!("Error creating element {e}");
            Err(StatusCode::FORBIDDEN)
        }
    }
}
