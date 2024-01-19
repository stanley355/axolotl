use crate::app_state::AppState;
use axum::{routing::post, Router};

use super::model::register_user;

pub fn account_router(state: AppState) -> Router {
    Router::new()
        .route("/register", post(register_user))
        .with_state(state)
}
