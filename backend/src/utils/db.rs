use mongodb::{Database, options::ClientOptions, Client};
use std::env;

pub async fn connect_db() -> Database {
    let url = env::var("db").expect("MongoDb url is not set");
    let client_options = ClientOptions::parse(url).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    client.database("RustChat")
}