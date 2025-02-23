use axum::{Router, routing::get, routing::post};
use dotenv::dotenv;
use sqlx::postgres::{PgPoolOptions, PgRow};
use std::{env, sync::Arc};
use tracing::Subscriber;
mod common;
use common::signin;
use common::signup;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

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
        .with_state(pool.clone());

    let api_routes = Router::new().nest("/common", common_routes);
    // .nest("/user", user_routes)
    // .nest("/space", space_routes)
    // .nest("/admin", admin_routes)

    let app = Router::new().nest("/api/v1/", api_routes);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
