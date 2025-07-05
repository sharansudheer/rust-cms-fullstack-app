use axum::{
    extract::State,
    routing::get,
    Router,
};
use redis::AsyncCommands;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Create Redis client and connection
    let client = redis::Client::open().unwrap();
    let connection = client.get_tokio_connection_manager().await.unwrap();
    let shared_redis = Arc::new(connection);

    // Build app with shared state
    let app = Router::new()
        .route("/", get(handler))
        .with_state(shared_redis);

    // Run server
    let addr = SocketAddr::from(());
    println!("Listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn handler(
    State(mut redis): State<Arc<redis::aio::ConnectionManager>>,
) -> String {
    let _: () = redis.set("mykey", "value").await.unwrap();
    let val: String = redis.get("mykey").await.unwrap();
    format!("Redis says: {}", val)
}
