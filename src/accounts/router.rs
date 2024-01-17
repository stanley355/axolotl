use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

pub fn account_router() -> Router {
    Router::new().route("/register", post(todos_create))
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Register {
    email: String,
    password: String,
}

async fn todos_create(Json(input): Json<Register>) -> Json<Register> {
    Json(input)
}
