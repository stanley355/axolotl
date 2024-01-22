use crate::{
    accounts::{req::RegisterPayload, res::LoginRegisterResponseBody},
    app_state::AppState,
    error_res::{ErrorPayload, ErrorResponse},
};

use axum::{extract::State, http::StatusCode, Json};
use bcrypt::verify;
use entity::users;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

type LoginUserResponse = Result<(StatusCode, Json<LoginRegisterResponseBody>), ErrorResponse>;

pub async fn login_user(
    state: State<AppState>,
    Json(body): Json<RegisterPayload>,
) -> LoginUserResponse {
    let find_result = users::Entity::find()
        .filter(users::Column::Email.eq(&body.email))
        .one(&state.db_connection)
        .await;

    match find_result {
        Ok(model_option) => match model_option {
            Some(user_model) => {
                let password_valid = verify(body.password, &user_model.password).unwrap();

                match password_valid {
                    true => {
                        let login_response = LoginRegisterResponseBody::new(user_model);
                        Ok((StatusCode::OK, Json(login_response)))
                    }
                    false => {
                        let error = ErrorPayload::new_bad_request(
                            "Harap periksa kembali password dan email Anda".to_string(),
                        );
                        Err((StatusCode::BAD_REQUEST, Json(error)))
                    }
                }
            }
            None => {
                let error = ErrorPayload::new_bad_request("Pengguna tidak terdaftar".to_string());
                Err((StatusCode::BAD_REQUEST, Json(error)))
            }
        },
        Err(db_error) => {
            let error = ErrorPayload::new_server_error(db_error.to_string());
            Err((StatusCode::BAD_REQUEST, Json(error)))
        }
    }
}
