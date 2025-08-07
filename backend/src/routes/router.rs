use axum:: {
    Router, 
    routing::{get}
};
use mongodb::Database;

pub async fn create_router(db: Database) -> Router {
    Router::new()
        .route("/", get(|| async {"Trail Router"}))
        .with_state(db)
} 