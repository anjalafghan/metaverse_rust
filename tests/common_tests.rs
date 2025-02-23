use crate::common::{signin, signup};
use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use hyper::{Body, Request};
use serde_json::json;
use sqlx::PgPool;
use std::sync::Arc;
use tower::ServiceExt;
mod common;
async fn setup_test_app(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/signin", post(signin))
        .route("/signup", post(signup))
        .with_state(pool.clone())
}

#[tokio::test]
async fn test_signup_success() {
    let pool = Arc::new(
        PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap(),
    );
    let app = setup_test_app(pool).await;

    let req = Request::builder()
        .method("POST")
        .uri("/signup")
        .header("Content-Type", "application/json")
        .body(Body::from(
            json!({
                "username": "test_user",
                "password": "test_pass",
                "avatar_id": 1,
                "role": "user"
            })
            .to_string(),
        ))
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_signup_duplicate_user() {
    let pool = Arc::new(
        PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap(),
    );
    let app = setup_test_app(pool).await;

    let req = Request::builder()
        .method("POST")
        .uri("/signup")
        .header("Content-Type", "application/json")
        .body(Body::from(
            json!({
                "username": "existing_user",
                "password": "test_pass",
                "avatar_id": 1,
                "role": "user"
            })
            .to_string(),
        ))
        .unwrap();

    let _ = app.oneshot(req.clone()).await.unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_signin_success() {
    let pool = Arc::new(
        PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap(),
    );
    let app = setup_test_app(pool).await;

    let req = Request::builder()
        .method("POST")
        .uri("/signin")
        .header("Content-Type", "application/json")
        .body(Body::from(
            json!({
                "username": "existing_user",
                "password": "test_pass"
            })
            .to_string(),
        ))
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_signin_wrong_password() {
    let pool = Arc::new(
        PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap(),
    );
    let app = setup_test_app(pool).await;

    let req = Request::builder()
        .method("POST")
        .uri("/signin")
        .header("Content-Type", "application/json")
        .body(Body::from(
            json!({
                "username": "existing_user",
                "password": "wrong_pass"
            })
            .to_string(),
        ))
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}
