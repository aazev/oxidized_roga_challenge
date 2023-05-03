use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{error::BoxDynError, MySqlPool};

use crate::traits::{database::Database, persist::Persist};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserModel {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUserModel {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginModel {
    pub email: String,
    pub password: String,
}

#[async_trait]
impl Database<MySqlPool> for UserModel {
    type Connection = MySqlPool;
    type Model = Self;

    async fn get<'long>(id: u64, connection_pool: &'long Self::Connection) -> sqlx::Result<Self>
    where
        Self: 'long,
    {
        let user = sqlx::query_as!(
            UserModel,
            r#"
                SELECT id, name, email, password, created_at, updated_at
                FROM users
                WHERE id = ?
            "#,
            id
        )
        .fetch_one(connection_pool)
        .await?;
        Ok(user)
    }

    async fn list<'long>(connection_pool: &'long Self::Connection) -> sqlx::Result<Vec<Self>>
    where
        Self: 'long,
    {
        let result = sqlx::query_as!(
            UserModel,
            r#"
                SELECT id, name, email, password, created_at, updated_at
                FROM users
            "#
        )
        .fetch_all(connection_pool)
        .await?;
        Ok(result)
    }
}

#[async_trait]
impl Persist for UserModel {
    async fn insert<'long>(
        &'long self,
        database_connection: &'long Self::Connection,
    ) -> sqlx::Result<Self::Model>
    where
        Self: 'long,
    {
        let result = sqlx::query!(
            r#"
                    INSERT INTO users (name, email, password)
                    VALUES (?, ?, ?)
                    RETURNING id
                "#,
            &self.name,
            &self.email,
            &self.password,
        )
        .execute(database_connection)
        .await?;
        Ok(Self::get(result.last_insert_id() as u64, &database_connection).await?)
    }

    async fn update<'long>(
        &'long self,
        database_connection: &'long Self::Connection,
    ) -> sqlx::Result<Self::Model> {
        let result = sqlx::query!(
            r#"
                    UPDATE users
                    SET name = ?, email = ?, password = ?
                    WHERE id = ?
                "#,
            &self.name,
            &self.email,
            &self.password,
            &self.id,
        )
        .execute(database_connection)
        .await?;
        Ok(Self::get(result.last_insert_id() as u64, &database_connection).await?)
    }

    async fn delete<'long>(
        &'long self,
        database_connection: &'long Self::Connection,
    ) -> Result<(), BoxDynError> {
        let result = sqlx::query!(
            r#"
                    DELETE FROM users
                    WHERE id = ?
                "#,
            &self.id,
        )
        .execute(database_connection)
        .await?;
        match result.rows_affected() {
            1 => Ok(()),
            _ => Err("Error deleting user".into()),
        }
    }
}

impl TryFrom<NewUserModel> for UserModel {
    type Error = sqlx::error::Error;

    fn try_from(new_user: NewUserModel) -> Result<Self, Self::Error> {
        let user = UserModel {
            id: 0,
            name: new_user.name,
            email: new_user.email,
            password: new_user.password,
            created_at: Utc::now(),
            updated_at: None,
        };
        Ok(user)
    }
}
