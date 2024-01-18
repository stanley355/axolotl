use axum::Router;

use dotenv::dotenv;
use std::env;

mod accounts;
mod app_state;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let host = env::var("HOST").unwrap_or("localhost".to_string());
    let port = &env::var("PORT").unwrap_or("8000".to_string());
    let address = format!("{}:{}", host, port);
    println!("Server running on: {}", address);

    let router = Router::new().nest("/accounts", accounts::router::account_router());
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
