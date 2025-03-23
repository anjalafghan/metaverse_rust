use axum::{Router, middleware, routing::get, routing::post, routing::put};
use dotenv::dotenv;
use maps::{create_maps::create_map, get_map::get_map};
use space::{create_space::create_space, delete_space::delete_space, get_space::get_space};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::{env, sync::Arc};
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;
use worlds::{create_world::create_world, get_worlds::get_worlds};
mod admin_middleware;
mod auth_middleware;
mod common;
mod element;
mod maps;
mod space;
mod space_middleware;
mod user;
mod worlds;
use admin_middleware::admin_middleware;
use auth_middleware::auth_middleware;
use common::{signin, signup};
use element::{add_element, create_element, update_element};
use maps::create_maps;
// use maps::{create_map, get_map, get_maps};
use space::{create_space, delete_space, get_space};
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
        .route(
            "/create_avatar",
            post(create_avatar).layer(middleware::from_fn(admin_middleware)), // Middleware applied only here
        )
        .with_state(pool.clone());

    let user_routes = Router::new()
        .route("/metadata", post(metadata))
        .route("/avatars", get(get_avatars))
        .route("/metadata/bulk", post(get_metadata_bulk))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(pool.clone());

    let world_routes = Router::new()
        .route(
            "/create",
            post(create_world).layer(middleware::from_fn(admin_middleware)),
        )
        .route("/get_worlds", get(get_worlds))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(pool.clone());

    let space_routes = Router::new()
        .route(
            "/create",
            post(create_space).layer(middleware::from_fn(admin_middleware)),
        )
        .route(
            "/delete_space",
            post(delete_space).layer(middleware::from_fn(admin_middleware)),
        )
        .route("/get_space", post(get_space))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(pool.clone());

    let map_routes = Router::new()
        .route(
            "/create",
            post(create_map).layer(middleware::from_fn(admin_middleware)),
        )
        .route("/get_map", post(get_map))
        .layer(middleware::from_fn(auth_middleware))
        // .route("/get_maps", post(get_maps))
        .with_state(pool.clone());

    // let element_routes = Router::new()
    //     .route("/create", post(create_element))
    //     .route("/add", post(add_element))
    //     // .route("/delete", post(delete_element))
    //     .route("/update", put(update_element))
    //     .with_state(pool.clone());

    let api_routes = Router::new()
        .nest("/common", common_routes)
        .nest("/user", user_routes)
        .nest("/map", map_routes)
        .nest("/space", space_routes)
        .nest("/worlds", world_routes);
    //
    // .nest("/element", element_routes)
    // ;

    // .nest("/admin", admin_routes)

    let app = Router::new().nest("/api/v1/", api_routes);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
