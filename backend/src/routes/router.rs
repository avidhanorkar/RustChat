use axum::{
    Router,
    routing::{get, post, delete},
};
use mongodb::Database;
use std::sync::Arc;

use crate::controller::auth_controller::*;
use crate::controller::user_controller::*;

pub async fn create_router(db: Arc<Database>) -> Router {
    Router::new()
        // Test Route
        .route("/", get(|| async { "Trail Router" }))

        // Auth Routes
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        
        // User Routes
        .route("/api/user/getUser/{id}", get(get_user_by_id))
        .route("/api/user/getAll", get(get_all_user))
        .route("/api/user/search/{name}", get(search_by_name))
        .route("/api/user/delete/{id}", delete(delete_user))
        .with_state(db)
}
