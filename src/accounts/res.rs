use entity::users;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RegisterResponse {
    pub id: String,
    pub fullname: String,
    pub phone_number: String,
    pub email: String,
}

impl RegisterResponse {
    pub fn new(user_model: users::Model) -> Self {
        RegisterResponse {
            id: user_model.id.to_string(),
            fullname: user_model.fullname,
            phone_number: user_model.phone_number,
            email: user_model.email,
        }
    }
}
