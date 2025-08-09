use axum::{Json, extract::State, http::StatusCode};
use bcrypt::{DEFAULT_COST, hash, verify};
use bson::{doc, oid::ObjectId};
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header, encode};
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};

// Crates
use crate::models::user_model::User;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub msg: String,
    pub user_id: ObjectId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JWTClaims {
    user_id: ObjectId,
    exp: usize,
    iat: usize,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub struct LoginResponse {
    msg: String,
    id: ObjectId,
    token: String,
}

pub async fn register(
    State(db): State<Arc<Database>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let collection: Collection<User> = db.collection("user");

    if payload.name.is_empty() || payload.email.is_empty() || payload.password.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Name, email and password are required".to_string(),
        ));
    }

    let filter = doc! {
        "email": &payload.email,
    };

    match collection.find_one(filter).await {
        Ok(Some(_)) => {
            return Err((StatusCode::BAD_REQUEST, "Email already exists".to_string()));
        }
        Ok(None) => {
            let hashed: String = match hash(&payload.password, DEFAULT_COST) {
                Ok(hashed) => hashed,
                Err(e) => {
                    println!("Some error occured in hashing the password: {}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to hash password".to_string(),
                    ));
                }
            };

            let user = User {
                id: ObjectId::new(),
                name: payload.name,
                email: payload.email,
                password: hashed,
            };

            match collection.insert_one(&user).await {
                Ok(user_created) => {
                    return Ok(Json(AuthResponse {
                        msg: "User created Successfully".to_string(),
                        user_id: user_created.inserted_id.as_object_id().ok_or((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Failed to get user id".to_string(),
                        ))?,
                    }));
                }
                Err(e) => {
                    println!("Some error occured in inserting the user: {}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to insert user".to_string(),
                    ));
                }
            }
        }
        Err(e) => {
            println!("Some error occured in finding the user: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to find user".to_string(),
            ));
        }
    }
}

#[axum::debug_handler]
pub async fn login(
    State(db): State<Arc<Database>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let collection: Collection<User> = db.collection("user");

    if payload.email.is_empty() || payload.password.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Email and password cannot be empty".to_string(),
        ));
    }

    let filter = doc! { "email": payload.email };

    match collection.find_one(filter).await {
        Ok(Some(user)) => match verify(&payload.password, &user.password) {
            Ok(true) => {
                let user_id = user.id;

                const TOKEN_EXPIRY: i64 = 24;
                let now = Utc::now();
                let exp = now + chrono::Duration::hours(TOKEN_EXPIRY);

                let claims = JWTClaims {
                    user_id: user_id.clone(),
                    exp: exp.timestamp() as usize,
                    iat: now.timestamp() as usize,
                };

                match encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(
                        env::var("secret").expect("JWT Secret is not set!").as_ref(),
                    ),
                ) {
                    Ok(token) => {
                        return Ok(Json(LoginResponse {
                            msg: "User created Successfully".to_string(),
                            id: user_id,
                            token: token,
                        }));
                    }
                    Err(e) => {
                        print!("some error occured: {}", e);
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Internal Server Error".to_string(),
                        ));
                    }
                }
            }
            Ok(false) => {
                return Err((StatusCode::UNAUTHORIZED, "Invalid password".to_string()));
            }
            Err(e) => {
                println!("Error in finding the email: {}", e);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error in verifying the password".to_string(),
                ));
            }
        },
        Ok(None) => {
            return Err((StatusCode::NOT_FOUND, "Email not Found".to_string()));
        }
        Err(e) => {
            println!("Database error while finding user: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ));
        }
    }
}
