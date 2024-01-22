use super::model::{login_user, register_user};
use crate::app_state::AppState;
use axum::{routing::post, Router};

pub fn account_router(state: AppState) -> Router {
    Router::new()
        .route("/register/", post(register_user))
        .route("/login/", post(login_user))
        .with_state(state)
}
