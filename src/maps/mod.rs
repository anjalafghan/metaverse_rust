use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::{self, Row, postgres::PgRow, query, query_as};
use std::sync::Arc;
use tracing::error;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateSpaceResponse {
    space_id: i32,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct DefaultElements {
    element_id: String,
    x: i32,
    y: i32,
    is_static: bool,
}

#[derive(serde::Deserialize)]
pub struct CreateSpacePayload {
    pub name: String,
    pub width: i32,
    pub height: Option<i32>,
    pub map_id: Option<i32>,
    pub default_elements: Vec<DefaultElements>,
}
pub async fn create_map(
    State(pool): State<Arc<sqlx::PgPool>>,
    Json(payload): Json<CreateSpacePayload>,
) -> Result<Json<CreateSpaceResponse>, StatusCode> {
    let result = sqlx::query_scalar!(
        "INSERT INTO maps (id, width, height, name) VALUES ($1, $2, $3, $4) RETURNING id",
        payload.map_id,
        payload.width,
        payload.height,
        payload.name,
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

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetMapPayload {
    id: i32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetMapsResponse {
    id: i32,
    width: i32,
    height: i32,
    name: String,
}

// pub async fn get_map(
//     State(pool): State<Arc<sqlx::PgPool>>,
//     Json(payload): Json<GetMapPayload>,
// ) -> Result<Json<GetMapsResponse>, StatusCode> {
//     let response = sqlx::query!(
//         "SELECT id, height, width, name FROM maps WHERE id= $1",
//         payload.id
//     )
//     .fetch_one(&*pool)
//     .await;
//     match response {
//         Ok(res) => {
//             let name: String;
//             match res.name {
//                 Some(n) => name = n,
//                 None => name = String::from("None"),
//             }

//             Ok(Json(GetMapsResponse {
//                 id: res.id,
//                 width: res.width,
//                 height: res.height,
//                 name,
//             }))
//         }
//         Err(e) => Err(StatusCode::BAD_REQUEST),
//     }
// }

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetMapsFinalResponse {
    maps: Vec<GetMapsResponse>,
}

pub async fn get_maps(
    State(pool): State<Arc<sqlx::PgPool>>,
) -> Result<Json<GetMapsFinalResponse>, StatusCode> {
    let response = sqlx::query("SELECT id , height, width, name FROM maps")
        .fetch_all(&*pool)
        .await;
    match response {
        Ok(rows) => {
            let maps: Vec<GetMapsResponse> = rows
                .into_iter()
                .map(|row| GetMapsResponse {
                    id: row.get("id"),
                    name: row.get("name"),
                    width: row.get("width"),
                    height: row.get("height"),
                })
                .collect();
            Ok(Json(GetMapsFinalResponse { maps }))
        }
        Err(e) => {
            error!("Error in getting all maps: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
