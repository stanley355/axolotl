use crate::{
    accounts::{req::RegisterPayload, res::LoginRegisterResponseBody},
    app_state::AppState,
    error_res::{ErrorPayload, ErrorResponse},
};

use axum::{extract::State, http::StatusCode, Json};
use bcrypt::{hash, DEFAULT_COST};
use entity::users;
use regex::Regex;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

type RegisterUserResponse = Result<(StatusCode, Json<LoginRegisterResponseBody>), ErrorResponse>;

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

pub async fn find_and_insert_register_user(
    body: RegisterPayload,
    db_connection: &DatabaseConnection,
) -> RegisterUserResponse {
    let clean_phone_number = body.phone_number.trim_start_matches("0");
    let find_result = users::Entity::find()
        .filter(
            users::Column::PhoneNumber
                .eq(clean_phone_number)
                .or(users::Column::Email.eq(&body.email)),
        )
        .one(db_connection)
        .await;

    match find_result {
        Ok(model_option) => match model_option {
            Some(_) => {
                let error = ErrorPayload::new_bad_request("Pengguna sudah terdaftar".to_string());
                Err((StatusCode::BAD_REQUEST, Json(error)))
            }
            None => insert_register_user(body, db_connection).await,
        },
        Err(find_error) => {
            let error = ErrorPayload::new_bad_request(find_error.to_string());
            Err((StatusCode::BAD_REQUEST, Json(error)))
        }
    }
}

pub async fn register_user(
    state: State<AppState>,
    Json(body): Json<RegisterPayload>,
) -> RegisterUserResponse {
    let name_valid = Regex::new(r"^[a-zA-Z0-9. ]+$")
        .unwrap()
        .is_match(&body.fullname);
    let clean_phone_number = body.phone_number.trim_start_matches("0");
    let phone_number_valid = Regex::new(r"^((08)|8){1}[0-9]{1,15}$")
        .unwrap()
        .is_match(clean_phone_number);
    let email_valid = Regex::new(r"^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w\w+)+$")
        .unwrap()
        .is_match(&body.email);

    match (name_valid, phone_number_valid, email_valid) {
        (false, ..) => {
            let error = ErrorPayload::new_bad_request("Nama Lengkap Invalid".to_string());
            return Err((StatusCode::BAD_REQUEST, Json(error)));
        }
        (_, false, _) => {
            let error = ErrorPayload::new_bad_request("Nomor Telepon Invalid".to_string());
            return Err((StatusCode::BAD_REQUEST, Json(error)));
        }
        (.., false) => {
            let error = ErrorPayload::new_bad_request("Email Invalid".to_string());
            return Err((StatusCode::BAD_REQUEST, Json(error)));
        }
        _ => find_and_insert_register_user(body, &state.db_connection).await,
    }
}
