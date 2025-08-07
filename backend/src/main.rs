use axum::{self, Router};
use dotenvy::dotenv;
use std::{
    net::SocketAddr,
    env
};


// mods
mod routes;

// crates
use routes::router::create_router;

#[tokio::main] 
async fn main(){

    dotenv().ok();
    let port: u16 = env::var("Port").expect("Port is not set").parse().expect("The port is not a number");
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("The server is up on address: {}", addr);

    let app: Router = create_router().await;

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}