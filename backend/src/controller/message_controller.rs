use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use bson::{doc, oid::ObjectId};
use futures_util::TryStreamExt;
use futures_util::stream::StreamExt;
use mongodb::{Collection, Database, bson::DateTime};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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

#[derive(Serialize)]
pub struct GetRoomMessages {
    sender_id: ObjectId,
    room_id: Option<ObjectId>,
    content: String,
}

#[derive(Serialize)]
pub struct GetDMMessages {
    sender_id: ObjectId,
    receiver_id: Option<ObjectId>,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentChat {
    pub chat_id: ObjectId,
    pub chat_type: String, // "user" or "room"
    pub name: String,
    pub last_message: String,
    pub timestamp: DateTime,
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

pub async fn get_messages_by_room_id(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
) -> Result<Json<Vec<GetRoomMessages>>, (StatusCode, String)> {
    let collection: Collection<Message> = db.collection("message");
    let room_obj_id = ObjectId::parse_str(id)
        .map_err(|_| (StatusCode::NOT_FOUND, "Wrong Room Id".to_string()))?;

    let filter = doc! {
        "room_id": &room_obj_id
    };

    let mut cursor = match collection.find(filter).await {
        Ok(cursor) => cursor,
        Err(e) => {
            println!("Some error occured: {e}");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ));
        }
    };

    let mut messages = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(message) => {
                messages.push(GetRoomMessages {
                    sender_id: message.sender_id,
                    room_id: message.room_id,
                    content: message.content,
                });
            }
            Err(e) => {
                println!("Some error occured: {e}");
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                ));
            }
        }
    }
    Ok(Json(messages))
}

pub async fn get_messages_in_dm(
    State(db): State<Arc<Database>>,
    claims: Claims,
    Path(id): Path<String>,
) -> Result<Json<Vec<GetDMMessages>>, (StatusCode, String)> {
    let collection: Collection<Message> = db.collection("message");
    let receiver_obj_id = ObjectId::parse_str(id)
        .map_err(|_| (StatusCode::NOT_FOUND, "Wrong Room Id".to_string()))?;

    let filter = doc! {
        "receiver_id": &receiver_obj_id,
        "sender_id": &claims.user_id
    };

    let mut cursor = match collection.find(filter).await {
        Ok(cursor) => cursor,
        Err(e) => {
            println!("Some error occured: {e}");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ));
        }
    };

    let mut messages = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(message) => messages.push(GetDMMessages {
                sender_id: message.sender_id,
                receiver_id: message.receiver_id,
                content: message.content,
            }),
            Err(e) => {
                println!("Some error occured: {e}");
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                ));
            }
        }
    }
    Ok(Json(messages))
}

pub async fn delete_message_in_room(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<String, (StatusCode, String)> {
    let collection: Collection<Message> = db.collection("message");
    let room_collection: Collection<Room> = db.collection("room");

    let message_obj_id = ObjectId::parse_str(id)
        .map_err(|_| (StatusCode::NOT_FOUND, "Wrong Room Id".to_string()))?;

    let user_id = claims.user_id;

    let filter = doc! {"_id": message_obj_id};

    match collection.find_one(filter.clone()).await {
        Ok(Some(message_found)) => {
            if message_found.sender_id == user_id {
                collection.delete_one(filter.clone()).await.map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal Server Error".to_string(),
                    )
                })?;
                return Ok("Message is deleted successfully by the sender himself".to_string());
            } else {
                match room_collection
                    .find_one(doc! {"_id": message_found.room_id})
                    .await
                {
                    Ok(Some(room)) => {
                        if room.owner == user_id {
                            collection.delete_one(filter.clone()).await.map_err(|_| {
                                (
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    "Internal Server Error".to_string(),
                                )
                            })?;
                            return Ok("Message Deleted Successfully by the owner".to_string());
                        } else {
                            return Err((
                                StatusCode::FORBIDDEN,
                                "You don't have permission to delete this message".to_string(),
                            ));
                        }
                    }
                    Ok(None) => {
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Internal Server Error".to_string(),
                        ));
                    }
                    Err(e) => {
                        println!("Some Error Occured: {e}");
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Internal Server Error".to_string(),
                        ));
                    }
                }
            }
        }
        Ok(None) => return Err((StatusCode::NOT_FOUND, "messagenot found".to_string())),
        Err(e) => {
            println!("Some error occured: {e}");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ));
        }
    }
}

pub async fn delete_message_in_dm(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<String, (StatusCode, String)> {
    let user_id = claims.user_id;
    let collection: Collection<Message> = db.collection("message");
    let message_id =
        ObjectId::parse_str(id).map_err(|_| (StatusCode::NOT_FOUND, "Id not found".to_string()))?;
    let filter = doc! {
        "_id": message_id
    };

    match collection.find_one(filter.clone()).await {
        Ok(Some(message)) => {
            if message.sender_id == user_id {
                collection.delete_one(filter).await.map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal Server Error".to_string(),
                    )
                })?;
                return Ok("the message deleted successfully".to_string());
            } else {
                return Err((
                    StatusCode::FORBIDDEN,
                    "You have no right to delete the message".to_string(),
                ));
            }
        }
        Ok(None) => {
            return Err((StatusCode::NOT_FOUND, "Message Not Found".to_string()));
        }
        Err(e) => {
            println!("Some Error Occured: {e}");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ));
        }
    }
}

pub async fn get_users_with_recent_chats(
    State(db): State<Arc<Database>>,
    Path(current_user_id): Path<String>,
) -> Result<Json<Vec<RecentChat>>, (StatusCode, String)> {
    let messages: Collection<mongodb::bson::Document> = db.collection("message");

    let parsed_user_id = ObjectId::parse_str(&current_user_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid user ID".to_string()))?;

    let pipeline = vec![
        // Get messages where the current user is either sender, receiver, or in a room
        doc! {
            "$match": {
                "$or": [
                    { "sender_id": &parsed_user_id },
                    { "receiver_id": &parsed_user_id },
                    { "room_id": { "$ne": null } }
                ]
            }
        },
        // Sort newest first
        doc! { "$sort": { "timestamp": -1 } },
        // Group: direct chats by "other user", rooms by "room_id"
        doc! {
            "$group": {
                "_id": {
                    "$cond": [
                        { "$ifNull": ["$room_id", false] }, // if room_id exists
                        { "type": "room", "id": "$room_id" },
                        { "type": "user",
                          "id": {
                              "$cond": [
                                  { "$eq": ["$sender_id", &parsed_user_id] },
                                  "$receiver_id",
                                  "$sender_id"
                              ]
                          }
                        }
                    ]
                },
                "last_message": { "$first": "$content" },
                "timestamp": { "$first": "$timestamp" }
            }
        },
        // Lookup the name: if type is "user" → from "user", else from "room"
        doc! {
            "$lookup": {
                "from": "user",
                "localField": "_id.id",
                "foreignField": "_id",
                "as": "user_info"
            }
        },
        doc! {
            "$lookup": {
                "from": "room",
                "localField": "_id.id",
                "foreignField": "_id",
                "as": "room_info"
            }
        },
        // Decide name field based on type
        doc! {
            "$addFields": {
                "chat_type": "$_id.type",
                "chat_id": "$_id.id",
                "name": {
                    "$cond": [
                        { "$eq": ["$_id.type", "user"] },
                        { "$arrayElemAt": ["$user_info.name", 0] },
                        { "$arrayElemAt": ["$room_info.name", 0] }
                    ]
                }
            }
        },
        // Cleanup
        doc! {
            "$project": {
                "_id": 0,
                "chat_id": 1,
                "chat_type": 1,
                "name": 1,
                "last_message": 1,
                "timestamp": 1
            }
        },
        // Sort by latest again
        doc! { "$sort": { "timestamp": -1 } },
    ];

    let mut cursor = messages.aggregate(pipeline).await.map_err(|e| {
        eprintln!("Error fetching recent chats: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to fetch recent chats".to_string(),
        )
    })?;

    let mut results = Vec::new();
    while let Some(doc) = cursor.next().await {
        if let Ok(d) = doc {
            if let Ok(chat) = mongodb::bson::from_document::<RecentChat>(d) {
                results.push(chat);
            }
        }
    }

    Ok(Json(results))
}

pub async fn get_messages_between_users(
    State(db): State<Arc<Database>>,
    Path((user1_id, user2_id)): Path<(String, String)>,
) -> Result<Json<Vec<Message>>, StatusCode> {
    // Convert both path params to ObjectId
    let user1_oid = ObjectId::parse_str(&user1_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let user2_oid = ObjectId::parse_str(&user2_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let collection = db.collection::<Message>("message");

    // MongoDB aggregation pipeline
    let pipeline = vec![
    doc! {
        "$match": {
            "$or": [
                { "$and": [ { "sender_id": user1_oid.clone() }, { "receiver_id": user2_oid.clone() } ] },
                { "$and": [ { "sender_id": user2_oid.clone() }, { "receiver_id": user1_oid.clone() } ] }
            ]
        }
    },
    doc! {
        "$sort": { "timestamp": 1 }
    }
];

    let mut cursor = collection
        .aggregate(pipeline)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut messages: Vec<Message> = Vec::new();
    while let Some(doc) = cursor
        .try_next()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        if let Ok(message) = bson::from_document::<Message>(doc) {
            messages.push(message);
        }
    }

    Ok(Json(messages))
}

pub async fn get_messages_in_room(
    State(db): State<Arc<Database>>,
    Path(room_id): Path<String>,
) -> Result<Json<Vec<Message>>, StatusCode> {
    // Convert room_id string to ObjectId
    let room_oid = ObjectId::parse_str(&room_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let collection: Collection<Message> = db.collection("message");

    // MongoDB aggregation pipeline
    let pipeline = vec![
        doc! {
            "$match": {
                "room_id": room_oid.clone()
            }
        },
        doc! {
            "$sort": { "timestamp": 1 }
        }
    ];

    let mut cursor = collection
        .aggregate(pipeline)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut messages: Vec<Message> = Vec::new();
    while let Some(doc) = cursor
        .try_next()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        if let Ok(message) = bson::from_document::<Message>(doc) {
            messages.push(message);
        }
    }

    Ok(Json(messages))
}