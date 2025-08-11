use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use bson::{doc, oid::ObjectId};
use chrono::{NaiveTime, TimeZone, Utc};
use mongodb::{Collection, Database, bson::DateTime};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::SystemTime};

// Crates
use crate::{
    middleware::auth_middleware::Claims,
    models::{message_model::Message, room_model::Room, user_model::User},
};

// DTOs
#[derive(Deserialize)]
pub struct MessageRequest {
    content: String,
}

#[derive(Serialize)]
pub struct MessageResponse {
    msg: String,
    id: String,
}

pub async fn send_message(
    State(db): State<Arc<Database>>,
    claims: Claims,
    Path(id): Path<String>,
    Json(payload): Json<MessageRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, String)> {
    let user_collection: Collection<User> = db.collection("user");
    let message_collection: Collection<Message> = db.collection("message");
    let room_collection: Collection<Room> = db.collection("room");

    let user_obj_id: ObjectId = claims.user_id.clone();

    if id.is_empty() || payload.content.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "The fields are required".to_string(),
        ));
    }

    let receiver_obj_id = ObjectId::parse_str(id)
        .map_err(|_| (StatusCode::NOT_FOUND, "Invalid Receiver Obj Id".to_string()))?;

    let filter = doc! {
        "_id": receiver_obj_id
    };

    enum Receiver {
        Room(Room),
        User(User),
    }

    let receiver = match room_collection.find_one(filter.clone()).await {
        Ok(Some(room)) => Receiver::Room(room),
        Ok(None) => match user_collection.find_one(filter).await {
            Ok(Some(user)) => Receiver::User(user),
            Ok(None) => return Err((StatusCode::NOT_FOUND, "Not Found".to_string())),
            Err(e) => {
                println!("Some error occurred: {e}");
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                ));
            }
        },
        Err(e) => {
            println!("Some error occured: {e}");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ));
        }
    };

    let (receiver_id, room_id) = match &receiver {
        Receiver::User(user) => (Some(user.id.clone()), None),
        Receiver::Room(room) => (None, Some(room.id.clone())),
    };

    let bson_datetime = DateTime::now();

    let new_message = Message {
        id: ObjectId::new(),
        sender_id: user_obj_id,
        receiver_id,
        room_id,
        content: payload.content,
        timestamp: bson_datetime,
    };

    match message_collection.insert_one(new_message).await {
        Ok(message_sent) => {
            return Ok(Json(MessageResponse {
                msg: "Message was sent Successfully".to_string(),
                id: message_sent
                    .inserted_id
                    .as_object_id()
                    .map(|oid| oid.to_hex())
                    .unwrap_or_default(),
            }));
        }
        Err(e) => {
            println!("Some error occurred: {e}");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ));
        }
    }
}
