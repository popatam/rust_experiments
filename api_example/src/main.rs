mod api;
mod dto;
mod error;
mod logger;
mod repo;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    logger::init();

    let dbpool = repo::pg::init_dbpool()
        .await
        .expect("couldn't initialize DB pool");

    let router = api::router::create_router(dbpool);

    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8000".to_string());
    let listener = TcpListener::bind(&bind_addr)
        .await
        .unwrap_or_else(|e| panic!("Can not bind to {}: {}", &bind_addr, e));

    axum::serve(listener, router)
        .await
        .unwrap_or_else(|e| panic!("Failed to start: {}", e));
}
