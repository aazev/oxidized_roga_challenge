use crate::traits::{database::Database, login::Login, persist::Persist, token::Token};
use anyhow::bail;
use argon2::{password_hash::SaltString, Argon2, PasswordHasher, PasswordVerifier};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use http::StatusCode;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sqlx::{error::BoxDynError, FromRow, MySqlPool};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, FromRow)]
pub struct UserModel {
    pub id: u64,
    pub api_token: String,
    #[schema(example = "John Doe")]
    pub name: String,
    #[schema(example = "john.doe@example.com")]
    pub email: String,
    #[serde(skip_serializing)]
    #[schema(example = "password")]
    password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NewUserModel {
    #[schema(example = "John Doe")]
    pub name: String,
    #[schema(example = "john.doe@example.com")]
    pub email: String,
    #[schema(example = "password")]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginModel {
    #[schema(example = "John Doe")]
    pub email: String,
    #[schema(example = "password")]
    pub password: String,
}

impl UserModel {
    pub fn get_password(&self) -> &str {
        &self.password
    }
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
                SELECT id, api_token, name, email, password, created_at, updated_at
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
                SELECT id, api_token, name, email, password, created_at, updated_at
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
                    INSERT INTO users (name, email, password, api_token)
                    VALUES (?, ?, ?, ?)
                "#,
            &self.name,
            &self.email,
            &self.password,
            &self.api_token.to_string(),
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

#[async_trait]
impl Login for UserModel {
    async fn login(body: LoginModel, connection_pool: &Self::Connection) -> anyhow::Result<Self> {
        let user = sqlx::query_as!(
            UserModel,
            r#"
                SELECT id, api_token, name, email, password, created_at, updated_at
                FROM users
                WHERE email = ?
            "#,
            body.email,
        )
        .fetch_one(connection_pool)
        .await?;

        if !UserModel::verify_password(&body.password, user.get_password()) {
            bail!(StatusCode::UNAUTHORIZED);
        }

        Ok(user)
    }

    async fn set_password(
        &mut self,
        password: String,
        connection_pool: &Self::Connection,
    ) -> anyhow::Result<()> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default().hash_password(password.as_bytes(), &salt);
        match password_hash {
            Ok(hash) => self.password = hash.to_string(),
            Err(_) => bail!(StatusCode::INTERNAL_SERVER_ERROR),
        }

        let _ = self.update(connection_pool).await;

        Ok(())
    }

    fn verify_password<'long>(password: &str, hash: &str) -> bool {
        let parsed_hash = argon2::PasswordHash::new(hash).unwrap();
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}

#[async_trait]
impl Token for UserModel {
    async fn get_by_uuid<'long>(
        uuid: Uuid,
        connection_pool: &'long Self::Connection,
    ) -> sqlx::Result<Self>
    where
        Self: 'long,
    {
        let user = sqlx::query_as!(
            UserModel,
            r#"
                SELECT id, api_token, name, email, password, created_at, updated_at
                FROM users
                WHERE api_token = ?
            "#,
            uuid.to_string(),
        )
        .fetch_one(connection_pool)
        .await?;
        Ok(user)
    }
}

impl TryFrom<NewUserModel> for UserModel {
    type Error = sqlx::error::Error;

    fn try_from(new_user: NewUserModel) -> Result<Self, Self::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(&new_user.password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        let user = UserModel {
            id: 0,
            api_token: Uuid::new_v4().to_string(),
            name: new_user.name,
            email: new_user.email,
            password: password_hash,
            created_at: Utc::now(),
            updated_at: None,
        };
        Ok(user)
    }
}
