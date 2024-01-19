use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorResponse {
  pub status: u16,
  pub message: String
}

impl ErrorResponse {
  pub fn new_bad_request(message: String) -> Self {
    ErrorResponse {
      status: StatusCode::BAD_REQUEST.as_u16(),
      message
    }
  }
}