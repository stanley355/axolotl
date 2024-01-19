use crate::app_state::AppState;
use axum::{extract::State, routing::post, Json, Router};

use super::req::RegisterPayload;

pub async fn register_user(state: State<AppState>, Json(req_body): Json<RegisterPayload>) -> Json<RegisterPayload> {
    println!("{:?}", req_body);

    Json(req_body)
}
