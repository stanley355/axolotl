use super::{req::RegisterPayload, res::RegisterResponse};
use crate::{
    app_state::AppState,
    error_res::{ErrorPayload, ErrorResponse},
};

use axum::{extract::State, http::StatusCode, Json};
use bcrypt::{hash, DEFAULT_COST};
use entity::users;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

type RegisterUserResult = Result<(StatusCode, Json<RegisterResponse>), ErrorResponse>;

pub async fn register_user(
    state: State<AppState>,
    Json(body): Json<RegisterPayload>,
) -> RegisterUserResult {
    let clean_phone_number = body.phone_number.trim_start_matches("0");

    let find_result = users::Entity::find()
        .filter(
            users::Column::PhoneNumber
                .eq(clean_phone_number)
                .or(users::Column::Email.eq(&body.email)),
        )
        .one(&state.db_connection)
        .await;

    match find_result {
        Ok(model_option) => match model_option {
            Some(_) => {
                let error = ErrorPayload::new_bad_request("Pengguna sudah terdaftar".to_string());
                Err((StatusCode::BAD_REQUEST, Json(error)))
            }
            None => insert_register_user(body, &state.db_connection).await,
        },
        Err(find_error) => {
            let error = ErrorPayload::new_bad_request(find_error.to_string());
            Err((StatusCode::BAD_REQUEST, Json(error)))
        }
    }
}

pub async fn insert_register_user(
    body: RegisterPayload,
    db_connection: &DatabaseConnection,
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
        .exec_with_returning(db_connection)
        .await;

    match insert_res {
        Ok(user_model) => {
            let register_response = RegisterResponse::new(user_model);
            Ok((StatusCode::CREATED, Json(register_response)))
        }
        Err(insert_error) => {
            let error = ErrorPayload::new_bad_request(insert_error.to_string());
            Err((StatusCode::BAD_REQUEST, Json(error)))
        }
    }
}
