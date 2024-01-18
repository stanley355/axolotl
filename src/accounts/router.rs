use axum::{routing::post, Json, Router, extract::State};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;

pub fn account_router(state: AppState) -> Router {
    Router::new().route("/register", post(todos_create)).with_state(state)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Register {
    email: String,
    password: String,
}

async fn todos_create(state: State<AppState>, Json(input): Json<Register>) -> Json<Register> {
    Json(input)
}
