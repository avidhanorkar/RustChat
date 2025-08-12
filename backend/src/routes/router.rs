use axum::{
    http::{HeaderValue, Method}, middleware::{from_fn, from_fn_with_state}, routing::{delete, get, post, put}, Router
};
use mongodb::Database;
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};

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
        .route("/api/message/delete/{id}", delete(delete_message_in_room))
        .route("/api/message/deleteDM/{id}", delete(delete_message_in_dm))
        .layer(from_fn(auth_middleware));


    let message_routes = Router::new()
        .route("/api/message/send/{id}", post(send_message))
        .route("/api/messages/getRoomMessages/{id}", get(get_messages_by_room_id))
        .route("/api/messages/getDM/{id}", get(get_messages_in_dm))
        .layer(from_fn_with_state(db.clone(), in_room))
        .layer(from_fn(auth_middleware));

    let origin = HeaderValue::from_str("http://localhost:3000").expect("Invalid header Value");

    let cors = CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    public_routes
        .merge(protected_routes)
        .merge(message_routes)
        .with_state(db)
        .layer(cors)
}
