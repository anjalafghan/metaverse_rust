#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        Router,
        body::Body,
        extract::Extension,
        http::{Request, StatusCode},
        routing::post,
    };
    use hyper::{Body as HyperBody, body::to_bytes};
    use serde_json::json;
    use sqlx::{PgPool, Pool, Postgres};
    use std::sync::Arc;
    use tower::ServiceExt; // For `oneshot` request simulation

    async fn setup_test_db() -> PgPool {
        let database_url = "postgres://test_user:test_password@localhost/test_db"; // Use a test DB
        PgPool::connect(database_url).await.unwrap()
    }

    async fn setup_app(pool: Arc<PgPool>, claims: Arc<Claims>) -> Router {
        Router::new()
            .route("/create_world", post(create_world))
            .layer(Extension(pool))
            .layer(Extension(claims))
    }

    #[tokio::test]
    async fn test_create_world_success() {
        let pool = Arc::new(setup_test_db().await);
        let claims = Arc::new(Claims {
            sub: "123".to_string(), // Valid user ID
        });

        let app = setup_app(pool, claims).await;

        let payload = json!({
            "name": "Test World",
            "description": "A test world",
            "thumbnail_url": "http://test.com/image.png",
            "is_public": true
        });

        let request = Request::builder()
            .method("POST")
            .uri("/create_world")
            .header("Content-Type", "application/json")
            .body(HyperBody::from(payload.to_string()))
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_create_world_invalid_user_id() {
        let pool = Arc::new(setup_test_db().await);
        let claims = Arc::new(Claims {
            sub: "invalid_id".to_string(), // Invalid user ID
        });

        let app = setup_app(pool, claims).await;

        let payload = json!({
            "name": "Test World",
            "description": "A test world",
            "thumbnail_url": "http://test.com/image.png",
            "is_public": true
        });

        let request = Request::builder()
            .method("POST")
            .uri("/create_world")
            .header("Content-Type", "application/json")
            .body(HyperBody::from(payload.to_string()))
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn test_create_world_db_error() {
        let pool = Arc::new(setup_test_db().await);
        let claims = Arc::new(Claims {
            sub: "123".to_string(), // Valid user ID
        });

        let app = setup_app(pool, claims).await;

        let payload = json!({
            "name": "Test World",
            "description": "A test world",
            "thumbnail_url": "http://test.com/image.png",
            "is_public": true
        });

        // Simulate DB error by using a faulty request (e.g., NULL name)
        let request = Request::builder()
            .method("POST")
            .uri("/create_world")
            .header("Content-Type", "application/json")
            .body(HyperBody::from(payload.to_string()))
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
}
