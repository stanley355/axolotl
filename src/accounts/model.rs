use super::{req::RegisterPayload, res::LoginRegisterResponseBody};
use crate::{
    app_state::AppState,
    error_res::{ErrorPayload, ErrorResponse},
};

use axum::{extract::State, http::StatusCode, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use entity::users;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTokenPayload {
    pub id: String,
    pub fullname: String,
    pub email: String,
    pub phone_number: String,
}

type RegisterUserResponse = Result<(StatusCode, Json<LoginRegisterResponseBody>), ErrorResponse>;

pub async fn register_user(
    state: State<AppState>,
    Json(body): Json<RegisterPayload>,
) -> RegisterUserResponse {
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
) -> RegisterUserResponse {
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
            let register_response = LoginRegisterResponseBody::new(user_model);
            Ok((StatusCode::CREATED, Json(register_response)))
        }
        Err(insert_error) => {
            let error = ErrorPayload::new_bad_request(insert_error.to_string());
            Err((StatusCode::BAD_REQUEST, Json(error)))
        }
    }
}

type LoginUserResponse = Result<(StatusCode, Json<LoginRegisterResponseBody>), ErrorResponse>;

pub async fn login_user(
    state: State<AppState>,
    Json(body): Json<RegisterPayload>,
) -> LoginUserResponse {
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
            Some(user_model) => {
                let password_valid = verify(body.password, &user_model.password).unwrap();

                match password_valid {
                    true => {
                        let login_response = LoginRegisterResponseBody::new(user_model);

                        Ok((StatusCode::ACCEPTED, Json(login_response)))
                    }
                    false => {
                        let error =
                            ErrorPayload::new_bad_request("Pengguna sudah terdaftar".to_string());
                        Err((StatusCode::BAD_REQUEST, Json(error)))
                    }
                }
            }
            None => {
                let error = ErrorPayload::new_bad_request("Pengguna sudah terdaftar".to_string());
                Err((StatusCode::BAD_REQUEST, Json(error)))
            }
        },
        Err(db_error) => {
            let error = ErrorPayload::new_server_error(db_error.to_string());
            Err((StatusCode::BAD_REQUEST, Json(error)))
        }
    }
}
