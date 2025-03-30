use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use std::sync::Arc;
use tracing::error;

#[derive(Deserialize)]
pub struct CreateElementTemplatePayload {
    name: String,
    element_type: ElementType,
    image_url: String,
    model_url: String,
    width: i32,
    height: i32,
    is_collidable: bool,
    interaction_data: serde_json::Value,
    physics_properties: serde_json::Value,
}

#[derive(sqlx::Type, serde::Serialize, serde::Deserialize, Debug)]
#[sqlx(type_name = "element_type_enum", rename_all = "lowercase")]
pub enum ElementType {
    Static,
    Interactive,
    Decorative,
    Portal,
}

pub async fn create_element_template(
    State(pool): State<Arc<sqlx::PgPool>>,
    Json(payload): Json<CreateElementTemplatePayload>,
) -> Result<StatusCode, StatusCode> {
    let element_type = match payload.element_type {
        ElementType::Static => "Static",
        ElementType::Interactive => "Interactive",
        ElementType::Decorative => "Decorative",
        ElementType::Portal => "Portal",
    };
    let response = sqlx::query(r#"INSERT INTO element_templates (name, type,name, type, image_url, model_url, width, height, is_collidable, interaction_data, physics_properties) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9"#)
        .bind(payload.name)
        .bind(element_type)
        .bind(payload.image_url)
        .bind(payload.model_url)
        .bind(payload.width)
        .bind(payload.height)
        .bind(payload.is_collidable)
        .bind(payload.interaction_data)
        .bind(payload.physics_properties)
        .execute(&*pool)
        .await;
    match response {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            error!("Error creating element templates {e}");
            Err(StatusCode::FORBIDDEN)
        }
    }
}
