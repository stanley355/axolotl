use std::fmt::Result;

use sea_orm::ModelTrait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiResponse {
  pub status: u32,
  pub message: String,
  pub data: Result
}