use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use bson::{doc, oid::ObjectId};
use futures_util::StreamExt;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

//crates
use crate::middleware::auth_middleware::Claims;
use crate::models::room_model::Room;
use crate::models::user_model::User;

// DTOs
#[derive(Deserialize)]
pub struct RoomRequest {
    name: String,
}

#[derive(Serialize)]
pub struct RoomResponse {
    msg: String,
    room_id: ObjectId,
}

#[derive(Serialize)]
pub struct Rooms {
    name: String,
    owner: ObjectId,
    participants: Vec<ObjectId>,
}

pub async fn create_room(
    State(db): State<Arc<Database>>,
    claims: Claims,
    Json(payload): Json<RoomRequest>,
) -> Result<Json<RoomResponse>, (StatusCode, String)> {
    let room_collection: Collection<Room> = db.collection("room");
    let user_collection: Collection<User> = db.collection("user");

    if payload.name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "All Fields are required".to_string(),
        ));
    }

    let user_obj_id = claims.user_id;

    let user_filter = doc! {
        "_id": &user_obj_id
    };

    let owner = match user_collection.find_one(user_filter).await {
        Ok(Some(owner)) => owner,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                "The owner with given user id not found".to_string(),
            ));
        }
        Err(e) => {
            println!("Error in finding the user: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ));
        }
    };

    let new_room = Room {
        id: ObjectId::new(),
        name: payload.name,
        owner: owner.id,
        participants: vec![owner.id],
    };

    match room_collection.insert_one(&new_room).await {
        Ok(result) => {
            return Ok(Json(RoomResponse {
                msg: format!(
                    "The room created successfully with the name {}",
                    new_room.name
                ),
                room_id: result.inserted_id.as_object_id().unwrap(),
            }));
        }
        Err(e) => {
            println!("Error in Creating a new room: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal SErver Error".to_string(),
            ));
        }
    }
}

pub async fn get_room(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
) -> Result<Json<Rooms>, (StatusCode, String)> {
    let collection: Collection<Room> = db.collection("room");

    if id.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Id is not given".to_string()));
    }

    let obj_id = ObjectId::parse_str(id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid ID format".to_string()))?;

    let filter = doc! {
        "_id": obj_id
    };

    match collection.find_one(filter).await {
        Ok(Some(room_found)) => Ok(Json(Rooms {
            name: room_found.name,
            owner: room_found.owner,
            participants: room_found.participants,
        })),
        Ok(None) => return Err((StatusCode::NOT_FOUND, "Room not found".to_string())),
        Err(e) => {
            println!("Error in finding the room: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database Error".to_string(),
            ));
        }
    }
}

pub async fn join_room(
    State(db): State<Arc<Database>>,
    claims: Claims,
    Path(room_id): Path<String>,
) -> Result<Json<RoomResponse>, (StatusCode, String)> {
    let user_obj_id = claims.user_id;
    let user_collection: Collection<User> = db.collection("user");
    let room_collection: Collection<Room> = db.collection("room");

    if room_id.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Fields are empty".to_string()));
    }

    let room_obj_id = ObjectId::parse_str(room_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid ID format".to_string()))?;

    let user_filter = doc! {
        "_id": &user_obj_id
    };

    let room_filter = doc! {
        "_id": &room_obj_id
    };

    if user_collection
        .find_one(user_filter)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB Error".to_string()))?
        .is_none()
    {
        return Err((StatusCode::NOT_FOUND, "User not found".to_string()));
    };

    let room = match room_collection.find_one(doc! { "_id": &room_obj_id }).await {
        Ok(Some(room)) => room,
        Ok(None) => return Err((StatusCode::NOT_FOUND, "Room not found".to_string())),
        Err(e) => {
            println!("Error in finding the room: {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "DB Error".to_string()));
        }
    };

    if room.participants.contains(&user_obj_id) {
        return Err((
            StatusCode::BAD_REQUEST,
            "User is already in the group".to_string(),
        ));
    }

    let room_update_filter = doc! {
        "$push": {
            "participants": &user_obj_id
        }
    };

    match room_collection
        .find_one_and_update(room_filter, room_update_filter)
        .await
    {
        Ok(_) => {
            return Ok(Json(RoomResponse {
                msg: format!("The user with id {}, has joined the room", user_obj_id),
                room_id: room_obj_id,
            }));
        }
        Err(e) => {
            println!("Some error occured: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ));
        }
    }
}

pub async fn get_all_rooms(
    State(db): State<Arc<Database>>,
) -> Result<Json<Vec<Rooms>>, (StatusCode, String)> {
    let collection: Collection<Room> = db.collection("room");

    let mut cursor = match collection.find(doc! {}).await {
        Ok(cursor) => cursor,
        Err(e) => {
            println!("There is some error in finding the rooms: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ));
        }
    };

    let mut rooms = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(room) => {
                rooms.push(Rooms {
                    name: room.name,
                    owner: room.owner,
                    participants: room.participants,
                });
            }
            Err(e) => {
                println!("Some error: {}", e);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internale Server Error".to_string(),
                ));
            }
        }
    }

    Ok(Json(rooms))
}

pub async fn leave_room(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<String, (StatusCode, String)> {
    let collection: Collection<Room> = db.collection("room");

    let room_obj_id = ObjectId::parse_str(&id)
        .map_err(|_| (StatusCode::NOT_FOUND, "Wrong Room Id".to_string()))?;

    // Step 1: Fetch the room
    let room = collection
        .find_one(doc! {"_id": &room_obj_id})
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            )
        })?
        .ok_or((StatusCode::NOT_FOUND, "Invalid Room Id".to_string()))?;

    // Step 2: Check ownership
    if room.owner == claims.user_id {
        return Err((
            StatusCode::BAD_REQUEST,
            "You are the owner of the room, you can't leave".to_string(),
        ));
    }

    // Step 3: Check membership
    if !room.participants.contains(&claims.user_id) {
        return Err((
            StatusCode::NOT_FOUND,
            "The user was never a part of the room".to_string(),
        ));
    }

    // Step 4: Remove user
    collection
        .update_one(
            doc! {"_id": &room_obj_id},
            doc! { "$pull": { "participants": &claims.user_id } },
        )
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            )
        })?;

    Ok("The user has successfully left the room".to_string())
}

pub async fn delete_room(
    State(db): State<Arc<Database>>,
    claims: Claims,
    Path(id): Path<String>,
) -> Result<String, (StatusCode, String)> {
    let collections: Collection<Room> = db.collection("room");

    let room_obj_id =
        ObjectId::parse_str(id).map_err(|_| (StatusCode::NOT_FOUND, "Invalid Id".to_string()))?;

    let room = match collections.find_one(doc! {"_id": &room_obj_id}).await {
        Ok(Some(room)) => room,
        Ok(None) => return Err((StatusCode::NOT_FOUND, "Room Not Found".to_string())),
        Err(e) => {
            println!("Some Error Occured: {e}");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ));
        }
    };

    if room.owner != claims.user_id {
        return Err((
            StatusCode::UNAUTHORIZED,
            "You have no right to delete the room".to_string(),
        ));
    }

    collections
        .delete_one(doc! {"_id": room_obj_id})
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".to_string(),
            )
        })?;

    return Ok("The room is deleted successfully by its owner".to_string());
}

