use axum::{
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", post(todos_create));

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Register {
    email: String,
    password: String
}

async fn todos_create(Json(input): Json<Register>) -> Json<Register> {

    Json(input)
}
