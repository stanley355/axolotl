use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorPayload {
  pub status: u16,
  pub message: String
}

impl ErrorPayload {
  pub fn new_bad_request(message: String) -> Self {
    ErrorPayload {
      status: StatusCode::BAD_REQUEST.as_u16(),
      message
    }
  }

  pub fn new_server_error(message: String) -> Self {
    ErrorPayload {
      status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
      message
    }
  }
}

pub type ErrorResponse = (StatusCode, Json<ErrorPayload>);