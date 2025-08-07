use axum::{self, Router};
use dotenvy::dotenv;
use std::{
    net::SocketAddr,
    env
};
use mongodb::Database;


// mods
mod routes;
mod utils;
mod models;

// crates
use routes::router::create_router;
use utils::db::connect_db;

#[tokio::main] 
async fn main(){

    dotenv().ok();
    let db: Database = connect_db().await;
    let port: u16 = env::var("Port").expect("Port is not set").parse().expect("The port is not a number");
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("The server is up on address: {}", addr);
    println!("MongoDB is Connected!!!");
    let app: Router = create_router(db).await;

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}