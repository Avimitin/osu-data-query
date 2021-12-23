use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
  pub api_key: String,
}

impl std::default::Default for AppConfig {
  fn default() -> Self {
    Self {
      api_key: String::new(),
    }
  }
}
