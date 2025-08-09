use axum::{
    middleware, routing::{delete, get, post}, Router
};
use mongodb::Database;
use std::sync::Arc;

use crate::controller::auth_controller::*;
use crate::controller::user_controller::*;
use crate::middleware::auth_middleware::*;

pub async fn create_router(db: Arc<Database>) -> Router {
    let public_routes = Router::new()
        .route("/", get(|| async { "Trail Router" }))
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login)).route("/api/user/getUser/{id}", get(get_user_by_id))
        .route("/api/user/getAll", get(get_all_user))
        .route("/api/user/search/{name}", get(search_by_name));

    let protected_routes = Router::new()
        .route("/api/user/delete/{id}", delete(delete_user))
        .layer(middleware::from_fn(auth_middleware)); // applied only here

    public_routes
        .merge(protected_routes)
        .with_state(db)
}

