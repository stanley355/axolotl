use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RegisterPayload {
    pub fullname: String,
    pub phone_number: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmailLoginPayload {
    pub email: String,
    pub password: String,
}