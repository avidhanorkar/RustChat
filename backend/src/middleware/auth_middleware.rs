use axum::{
    body::Body, extract::FromRequestParts, http::{Request, StatusCode}, middleware::Next, response::Response, Json
};
use serde::{Serialize, Deserialize};
use serde_json::json;
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::env;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Claims {
    pub user_id: String,
    pub username: String,
    pub exp: usize,
    pub iat: usize,
}

pub async fn auth_middleware (
    mut request:Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {

    let headers = request.headers();
    let auth_headers = headers.get("AUTHORIZATION").and_then(|header| header.to_str().ok());

    let token = match auth_headers {
        Some(token) => {
            if token.starts_with("Bearer ") {
                token.trim_start_matches("Bearer ")
            } else {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"error": "Invalid authorization header format"}))
                ));
            }
        } _ => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Missing or invalid authorization header"}))
            ));
        }
    };


    match decode::<Claims> (
        token,
        &DecodingKey::from_secret(env::var("JWT_SECRET").expect("JWT_SECRET not set").as_ref()),
        &Validation::default()
    ) {
        Ok(token_data) => {
            request.extensions_mut().insert(token_data.claims);
            Ok(next.run(request).await)
        } Err(e) => {
            println!("JWT decode error: {}", e);
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid token"}))
            ));
        }
    }
}

// Extrayctor to get the user data from the middleware
impl<S> FromRequestParts<S> for Claims 
where S: Send + Sync {
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut axum::http::request::Parts, _state: &S,) -> Result<Self, Self::Rejection> {
        if let Some(claims) = parts.extensions.get::<Claims>() {
            Ok(claims.clone())
        } else {
            Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Unauthorized"}))
            ))
        }
        
    }
}