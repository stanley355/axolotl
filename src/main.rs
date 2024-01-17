use axum::Router;

mod accounts;

#[tokio::main]
async fn main() {
    // build our application with a route

    let router = Router::new().nest("/accounts", accounts::router::account_router());

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
}
