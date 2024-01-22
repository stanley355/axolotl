use entity::users;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTokenPayload {
    pub id: String,
    pub fullname: String,
    pub email: String,
    pub phone_number: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoginRegisterResponseBody {
    token: String,
}

impl LoginRegisterResponseBody {
    pub fn new(user_model: users::Model) -> Self {
        let token_payload = UserTokenPayload {
            id: user_model.id.to_string(),
            fullname: user_model.fullname,
            email: user_model.email,
            phone_number: user_model.phone_number,
        };

        LoginRegisterResponseBody {
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
