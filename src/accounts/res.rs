use super::model::UserTokenPayload;
use entity::users;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoginRegisterResponse {
    token: String,
}

impl LoginRegisterResponse {
    pub fn new(user_model: users::Model) -> Self {
        let token_payload = UserTokenPayload {
            id: user_model.id.to_string(),
            fullname: user_model.fullname,
            email: user_model.email,
            phone_number: user_model.phone_number,
        };

        LoginRegisterResponse {
            token: Self::tokenize(token_payload),
        }
    }

    pub fn tokenize(token_payload: UserTokenPayload) -> String {
        encode(
            &Header::default(),
            &token_payload,
            &EncodingKey::from_secret("axolotl".as_ref()),
        )
        .unwrap_or_default()
    }
}
