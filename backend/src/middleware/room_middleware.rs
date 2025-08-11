use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use bson::{doc, oid::ObjectId};
use mongodb::{Collection, Database};

use crate::{middleware::auth_middleware::Claims, models::room_model::Room};

pub async fn in_room(
    State(db): State<Arc<Database>>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let claims = req
        .extensions()
        .get::<Claims>()
        .ok_or((StatusCode::UNAUTHORIZED, "Missing Claims".to_string()))?
        .clone();

    let path = req.uri().path();
    let segments: Vec<&str> = path.split('/').collect();
    let id = segments.last().unwrap_or(&"").to_string();

    let room_obj_id = ObjectId::parse_str(&id)
        .map_err(|_| (StatusCode::NOT_FOUND, "Invalid Room Id".to_string()))?;

    let user_obj_id = claims.user_id;

    let collection: Collection<Room> = db.collection("room");
    let filter = doc! { "_id": room_obj_id };

    match collection.find_one(filter).await {
        Ok(Some(room)) => {
            if room.participants.contains(&user_obj_id) {
                Ok(next.run(req).await)
            } else {
                Err((
                    StatusCode::UNAUTHORIZED,
                    "You are not part of the given Room".to_string(),
                ))
            }
        }
        Ok(None) => Ok(next.run(req).await),
        Err(e) => {
            println!("Some error occurred: {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ))
        }
    }
}
