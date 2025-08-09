use axum::{self, Router};
use dotenvy::dotenv;
use std::{
    net::SocketAddr,
    env,
    sync::Arc
};
use mongodb::Database;

// mods
mod routes;
mod utils;
mod models;
mod controller;
mod middleware;

// crates
use routes::router::create_router;
use utils::db::connect_db;

#[tokio::main] 
async fn main(){

    dotenv().ok();
    let port: u16 = env::var("Port").expect("Port is not set").parse().expect("The port is not a number");
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("The server is up on address: {}", addr);
    let db: Arc<Database> = Arc::new(connect_db().await.expect("Failed to Connect to MongoDb"));
    let app: Router = create_router(db).await;

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}