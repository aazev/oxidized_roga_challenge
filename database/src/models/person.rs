use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{error::BoxDynError, FromRow, MySqlPool};

use crate::traits::{database::Database, persist::Persist};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PersonModel {
    pub id: u64,
    pub name: String,
    pub mothers_name: String,
    pub fathers_name: String,
    pub cep: String,
    pub birth_date: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPersonModel {
    pub name: String,
    pub mothers_name: String,
    pub fathers_name: String,
    pub cep: String,
    pub birth_date: NaiveDate,
    pub age: u8,
}

#[async_trait]
impl Database<MySqlPool> for PersonModel {
    type Connection = MySqlPool;
    type Model = Self;

    async fn get<'long>(id: u64, connection: &'long Self::Connection) -> sqlx::Result<Self>
    where
        Self: 'long,
    {
        let person = sqlx::query_as!(PersonModel, "SELECT * FROM persons WHERE id = ?", id)
            .fetch_one(connection)
            .await?;

        Ok(person)
    }

    async fn list<'long>(connection: &'long Self::Connection) -> sqlx::Result<Vec<Self>>
    where
        Self: 'long,
    {
        let result = sqlx::query_as!(PersonModel, "SELECT * FROM persons")
            .fetch_all(connection)
            .await?;

        Ok(result)
    }
}

#[async_trait]
impl Persist for PersonModel {
    async fn insert<'long>(
        &'long self,
        database_connection: &'long Self::Connection,
    ) -> sqlx::Result<Self::Model>
    where
        Self: 'long,
    {
        let result = sqlx::query!(
            "INSERT INTO persons (name, mothers_name, fathers_name, cep, birth_date, created_at) VALUES (?, ?, ?, ?, ?, ?)",
            self.name,
            self.mothers_name,
            self.fathers_name,
            self.cep,
            self.birth_date,
            self.created_at,
        ).execute(database_connection).await?;
        Ok(Self::get(result.last_insert_id() as u64, &database_connection).await?)
    }

    async fn update<'long>(
        &'long self,
        database_connection: &'long Self::Connection,
    ) -> sqlx::Result<Self::Model>
    where
        Self: 'long,
    {
        let result = sqlx::query!(
            "UPDATE persons SET name = ?, mothers_name = ?, fathers_name = ?, cep = ?, birth_date = ?, updated_at = ? WHERE id = ?",
            self.name,
            self.mothers_name,
            self.fathers_name,
            self.cep,
            self.birth_date,
            self.updated_at,
            self.id,
        ).execute(database_connection).await?;
        Ok(Self::get(result.last_insert_id() as u64, &database_connection).await?)
    }

    async fn delete<'long>(
        &'long self,
        database_connection: &'long Self::Connection,
    ) -> Result<(), BoxDynError>
    where
        Self: 'long,
    {
        let result = sqlx::query!("DELETE FROM persons WHERE id = ?", self.id,)
            .execute(database_connection)
            .await?;
        match result.rows_affected() {
            1 => Ok(()),
            _ => Err("Error deleting person".into()),
        }
    }
}

impl TryFrom<NewPersonModel> for PersonModel {
    type Error = sqlx::error::Error;

    fn try_from(value: NewPersonModel) -> Result<Self, Self::Error> {
        let person = PersonModel {
            id: 0,
            name: value.name,
            mothers_name: value.mothers_name,
            fathers_name: value.fathers_name,
            cep: value.cep,
            birth_date: value.birth_date,
            created_at: Utc::now(),
            updated_at: None,
        };

        Ok(person)
    }
}
