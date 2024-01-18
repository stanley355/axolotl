use axum::Router;

mod accounts;
mod app_state;

#[tokio::main]
async fn main() {
    let router = Router::new().nest("/accounts", accounts::router::account_router());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
}
