use axum::middleware;
use axum::{Router, routing::get, routing::post};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::{env, sync::Arc};
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

mod auth_middleware;
mod common;
mod space;
mod space_middleware;
mod user;

use auth_middleware::auth_middleware;
use common::{signin, signup};
use space::create_space;
use space_middleware::space_middleware;
use user::{create_avatar, get_avatars, get_metadata_bulk, metadata};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("error setting subscriber");

    info!("Start Server");

    let database_url = env::var("DATABASE_URL").expect("No Database URL found");
    let max_connections: u32 = env::var("MAX_CONNECTIONS")
        .expect("No max connections found")
        .parse()
        .expect("MAX_CONNECTIONS should be an int");

    let pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(&database_url)
            .await?,
    );

    let common_routes = Router::new()
        .route("/signin", post(signin))
        .route("/signup", post(signup))
        .route("/create_avatar", post(create_avatar))
        .with_state(pool.clone());

    let user_routes = Router::new()
        .route("/metadata", post(metadata))
        .layer(middleware::from_fn(auth_middleware))
        .route("/avatars", get(get_avatars))
        .route("/metadata/bulk", post(get_metadata_bulk))
        .with_state(pool.clone());

    let space_routes = Router::new()
        .route("/create", post(create_space))
        .layer(middleware::from_fn(auth_middleware))
        .layer(middleware::from_fn(space_middleware))
        .with_state(pool.clone());

    let api_routes = Router::new()
        .nest("/common", common_routes)
        .nest("/user", user_routes)
        .nest("/space", space_routes);
    // .nest("/admin", admin_routes)

    let app = Router::new().nest("/api/v1/", api_routes);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
