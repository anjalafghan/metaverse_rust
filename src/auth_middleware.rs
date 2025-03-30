use axum::{
    body::Body,
    extract::Request,
    http::{Response, StatusCode},
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
    pub role: String,
}

static SECRET_KEY: Lazy<Vec<u8>> = Lazy::new(|| {
    dotenv().ok();
    env::var("SECRET_KEY_JWT")
        .expect("Error in getting secret key")
        .into_bytes()
});

use tracing::{error, info};

pub async fn auth_middleware(request: Request, next: Next) -> Result<Response<Body>, StatusCode> {
    info!("Authenticating request...");

    // Extract authorization header
    let token = match request
        .headers()
        .get("Authorization")
        .and_then(|val| val.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
    {
        Some(token) => token,
        None => {
            error!("Missing or invalid Authorization header");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Decode the token
    let token_data = match decode::<Claims>(
        token,
        &DecodingKey::from_secret(&*SECRET_KEY),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(data) => {
            info!("Token successfully decoded for user: {}", data.claims.sub);
            data
        }
        Err(err) => {
            error!("Invalid token: {:?}", err);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Create an Arc<Claims> to share across middleware
    let claims = Arc::new(token_data.claims);

    // Create a new request with the claims in the extensions
    let (mut parts, body) = request.into_parts();
    parts.extensions.insert(claims.clone());
    let new_request = Request::from_parts(parts, body);

    // Log the claims after insertion
    if let Some(retrieved_claims) = new_request.extensions().get::<Arc<Claims>>() {
        info!("Claims after insertion: {:?}", retrieved_claims);
    } else {
        error!("Failed to retrieve claims after insertion");
    }

    info!("Authentication successful, proceeding to next middleware...");

    // Pass the new request to the next middleware
    Ok(next.run(new_request).await)
}
