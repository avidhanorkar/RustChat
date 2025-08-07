use mongodb::{Database, options::ClientOptions, Client};
use std::env;

pub async fn connect_db() -> Result<Database, mongodb::error::Error> {
    let url = env::var("db").expect("MongoDB URL is not set in the environment variables");
    let client_options = ClientOptions::parse(&url).await?;
    let client = Client::with_options(client_options)?;
    println!("Successfully connected to MongoDB");

    Ok(client.database("RustChat"))
}
