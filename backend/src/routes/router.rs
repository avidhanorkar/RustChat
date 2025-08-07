use axum:: {
    Router, 
    routing::{get, post}
};
use std::sync::Arc;
use mongodb::Database;

use crate::controller::auth_controller::*;

pub async fn create_router(db: Arc<Database>) -> Router {
    Router::new()
        .route("/", get(|| async {"Trail Router"}))
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .with_state(db)
} 