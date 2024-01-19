use crate::{api_response::ApiResponse, app_state::AppState};
use axum::{extract::State, http::StatusCode, Json};
use entity::users::{self, Model};
use sea_orm::{ActiveModelTrait, ActiveValue, DbErr, EntityTrait};
use serde::Deserialize;

use super::req::RegisterPayload;

pub async fn register_user(
    state: State<AppState>,
    Json(body): Json<RegisterPayload>,
) -> (StatusCode, Json<Model>) {
    let new_user = users::ActiveModel {
        fullname: ActiveValue::NotSet,
        phone_number: ActiveValue::Set(body.phone_number),
        email: ActiveValue::Set(body.email),
        password: ActiveValue::Set(body.password),
        ..Default::default()
    };

    let res: Result<users::Model, sea_orm::prelude::DbErr> = users::Entity::insert(new_user)
        .exec_with_returning(&state.db_conection)
        .await;

    (StatusCode::BAD_GATEWAY, Json(res.unwrap()))
}
