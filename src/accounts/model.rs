use super::req::RegisterPayload;
use crate::app_state::AppState;

use axum::{extract::State, http::StatusCode, Json};
use entity::users;
use sea_orm::{ActiveValue, EntityTrait};

type RegisterUserResult = Result<(StatusCode, Json<users::Model>), (StatusCode, String)>;

pub async fn register_user(
    state: State<AppState>,
    Json(body): Json<RegisterPayload>,
) -> RegisterUserResult {
    let new_user = users::ActiveModel {
        fullname: ActiveValue::Set(body.fullname),
        phone_number: ActiveValue::Set(body.phone_number),
        email: ActiveValue::Set(body.email),
        password: ActiveValue::Set(body.password),
        ..Default::default()
    };

    let insert_res: Result<users::Model, sea_orm::prelude::DbErr> = users::Entity::insert(new_user)
        .exec_with_returning(&state.db_conection)
        .await;

    match insert_res {
        Ok(user_model) => Ok((StatusCode::CREATED, Json(user_model))),
        Err(db_error) => Err((StatusCode::BAD_REQUEST, db_error.to_string())),
    }
}
