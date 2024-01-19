use super::{req::RegisterPayload, res::RegisterResponse};
use crate::app_state::AppState;

use axum::{extract::State, http::StatusCode, Json};
use bcrypt::{hash, DEFAULT_COST};
use entity::users;
use sea_orm::{ActiveValue, EntityTrait};

type RegisterUserResult = Result<(StatusCode, Json<RegisterResponse>), (StatusCode, String)>;

pub async fn register_user(
    state: State<AppState>,
    Json(body): Json<RegisterPayload>,
) -> RegisterUserResult {
    let clean_phone_number = body.phone_number.trim_start_matches("0");
    let hash_password = hash(body.password, DEFAULT_COST).unwrap();
    let new_user = users::ActiveModel {
        fullname: ActiveValue::Set(body.fullname),
        phone_number: ActiveValue::Set(clean_phone_number.to_string()),
        email: ActiveValue::Set(body.email),
        password: ActiveValue::Set(hash_password),
        ..Default::default()
    };

    let insert_res: Result<users::Model, sea_orm::prelude::DbErr> = users::Entity::insert(new_user)
        .exec_with_returning(&state.db_conection)
        .await;

    match insert_res {
        Ok(user_model) => {
          let register_response = RegisterResponse::new(user_model);
          Ok((StatusCode::CREATED, Json(register_response)))
        },
        Err(db_error) => Err((StatusCode::BAD_REQUEST, db_error.to_string())),
    }
}
