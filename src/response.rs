use serde::{Serialize, Deserialize};
use crate::beatmaps::BeatMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OsuAPIResponse {
    BeatMapResp(Vec<BeatMap>),
    ErrorResp(ErrorResp),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResp {
    error: String,
}

impl ErrorResp {
    pub fn err(&self) -> &str {
        return &self.error;
    }
}
