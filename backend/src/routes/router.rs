use axum:: {
    Router, 
    routing::{get}
};

pub async fn create_router() -> Router {
    Router::new().
        route("/", get(|| async {"Trail Router"}))
} 