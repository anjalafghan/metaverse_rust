use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;
use sqlx::FromRow;
use std::sync::Arc;
use tracing::error;

#[derive(Serialize, FromRow)]
pub struct World {
    id: i32,
    name: String,
    description: String,
    thumbnail_url: String,
}
#[derive(serde::Deserialize, serde::Serialize)]
pub struct GetWorldResponse {}
pub async fn get_worlds(
    State(pool): State<Arc<sqlx::PgPool>>,
) -> Result<Json<Vec<World>>, StatusCode> {
    let response =
        sqlx::query_as::<_, World>("SELECT id, name, description, thumbnail_url FROM worlds")
            .fetch_all(&*pool)
            .await;
    match response {
        Ok(worlds) => Ok(Json(worlds)),
        Err(e) => {
            error!("Error fetching all worlds {}", e);
            return Err(StatusCode::FORBIDDEN);
        }
    }
}
