use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericMessage {
    pub status: u16,
    pub message: String,
}

impl GenericMessage {
    pub fn new(status: u16, message: String) -> Self {
        Self { status, message }
    }
}

impl Default for GenericMessage {
    fn default() -> Self {
        Self {
            status: 200,
            message: "Ok".to_string(),
        }
    }
}
