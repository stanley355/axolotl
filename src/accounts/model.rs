use crate::app_state::AppState;
use axum::{extract::State, Json};
use entity::users;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};

use super::req::RegisterPayload;

pub async fn register_user(
    state: State<AppState>,
    Json(body): Json<RegisterPayload>,
) -> Json<()> {
    let new_user = users::ActiveModel {
      id: ActiveValue::NotSet,
        fullname: ActiveValue::default(),
        phone_number: ActiveValue::Set(body.phone_number),
        email: ActiveValue::Set(body.email),
        password: ActiveValue::Set(body.password),
        created_at: ActiveValue::default(),
        updated_at: ActiveValue::default()
    };


    let res = users::Entity::insert(new_user).exec(&state.db_conection).await;

    println!("{:?}", res.unwrap());
    // match res {
    //    Ok(insert_result) => Json(insert_result),
    //    Err(_) => Json() 
    // }

    Json(())
}
