use axum::{
    body::{Body, Bytes},
    extract::Request,
    http::{HeaderMap, Response, StatusCode},
    middleware::Next,
};
use tracing::{error, info};

#[derive(serde::Deserialize, Clone, Debug)]
pub struct CreateSpacePayload {
    pub name: String,
    pub width: i32,
    pub height: Option<i32>,
    pub map_id: Option<String>,
}

pub async fn space_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    // Check content-type header
    if headers
        .get("content-type")
        .map(|h| {
            h.to_str()
                .map_or(true, |s| s.to_lowercase() != "application/json")
        })
        .unwrap_or(true)
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    let (parts, body) = request.into_parts();
    info!("parts are {:?} and body is {:?}", parts, body);

    let body_bytes_result: Result<Bytes, axum::Error> =
        axum::body::to_bytes(body, usize::MAX).await;

    let body_bytes = match body_bytes_result {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Error reading body: {:?}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let payload_result: Result<CreateSpacePayload, serde_json::Error> =
        serde_json::from_slice(&body_bytes);

    let payload = match payload_result {
        Ok(payload) => payload,
        Err(e) => {
            error!("Error parsing payload: {:?}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    if payload.width < 100 || payload.width > 1000 {
        error!("Got an error for width: {}", payload.width);
        return Err(StatusCode::BAD_REQUEST);
    }

    if let Some(height) = payload.height {
        if height < 100 || height > 1000 {
            error!("Got an error for height: {}", height);
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    let mut request = Request::from_parts(parts, Body::from(body_bytes));

    request.extensions_mut().insert(payload);

    Ok(next.run(request).await)
}
