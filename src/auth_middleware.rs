use axum::{
    body::Body,
    extract::Request,
    http::{HeaderMap, Response, StatusCode},
    middleware::Next,
};
use std::sync::Arc;

use dotenv::dotenv;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

static SECRET_KEY: Lazy<Vec<u8>> = Lazy::new(|| {
    dotenv().ok();
    env::var("SECRET_KEY_JWT")
        .expect("Error in getting secret key")
        .into_bytes()
});

pub async fn auth_middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let token = headers
        .get("Authorization")
        .and_then(|val| val.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(&*SECRET_KEY),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(Arc::new(token_data.claims));
    Ok(next.run(request).await)
}
