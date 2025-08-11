use axum::{
    Router, middleware::{from_fn, from_fn_with_state},
    routing::{delete, get, post, put},
};
use mongodb::Database;
use std::sync::Arc;

use crate::{
    controller::{
        auth_controller::*, message_controller::*, room_controller::*, user_controller::*,
    },
    middleware::{auth_middleware::*, room_middleware::*},
};

pub async fn create_router(db: Arc<Database>) -> Router {
    let public_routes = Router::new()
        .route("/", get(|| async { "Trail Router" }))
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/user/getUser/{id}", get(get_user_by_id))
        .route("/api/user/getAll", get(get_all_user))
        .route("/api/user/search/{name}", get(search_by_name))
        .route("/api/room/getAll", get(get_all_rooms));

    let protected_routes = Router::new()
        .route("/api/user/delete/{id}", delete(delete_user))
        .route("/api/room/create", post(create_room))
        .route("/api/room/{id}", get(get_room))
        .route("/api/room/join/{room_id}", put(join_room))
        .route("/api/room/leave/{id}", put(leave_room))
        .route("/api/room/delete/{id}", delete(delete_room))
        .layer(from_fn(auth_middleware));


    let message_routes = Router::new()
        .route("/api/message/send/{id}", post(send_message))
        .layer(from_fn_with_state(db.clone(), in_room))
        .layer(from_fn(auth_middleware));


    public_routes
        .merge(protected_routes)
        .merge(message_routes)
        .with_state(db)
}
