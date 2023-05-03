use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OkMessage {
    pub code: u16,
    pub message: String,
}

impl OkMessage {
    pub fn new(code: u16, message: String) -> Self {
        Self { code, message }
    }
}

impl Default for OkMessage {
    fn default() -> Self {
        Self {
            code: 200,
            message: "Ok".to_string(),
        }
    }
}
