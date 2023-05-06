use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::objects::{address::Address, annotation::Annotation};

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: u64,
    pub name: String,
    pub mothers_name: String,
    pub fathers_name: String,
    pub cep: String,
    pub address: Option<Address>,
    pub annotations: Vec<Annotation>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
