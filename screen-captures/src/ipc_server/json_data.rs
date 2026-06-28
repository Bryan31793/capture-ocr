use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct OcrRequest {
    pub action: String,
    pub payload: serde_json::Value,
}
    
#[derive(Deserialize)]
pub enum OcrResponse {
    Ok {
        data: serde_json::Value,
    },
    Error {
        code: String,
        message: String,
        retryable: Option<bool>,
    }
}

