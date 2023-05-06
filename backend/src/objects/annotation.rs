use database::models::annotation::AnnotationModel;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Annotation {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<AnnotationModel> for Annotation {
    fn from(annotation: AnnotationModel) -> Self {
        Self {
            id: annotation.id,
            title: annotation.title,
            description: annotation.description,
            created_at: annotation.created_at,
            updated_at: annotation.updated_at,
        }
    }
}
