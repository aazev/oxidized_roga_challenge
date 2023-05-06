use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::MySqlPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnotationModel {
    pub id: u64,
    pub person_id: u64,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl AnnotationModel {
    pub async fn list(person_id: u64, connection: &MySqlPool) -> Result<Vec<AnnotationModel>, sqlx::Error> {
        let annotations = sqlx::query_as!(
            AnnotationModel,
            r#"
            SELECT id, person_id, title, description, created_at, updated_at
            FROM annotations
            WHERE person_id = ?
            "#,
            person_id
        )
        .fetch_all(connection)
        .await?;
        Ok(annotations)
    }
}
