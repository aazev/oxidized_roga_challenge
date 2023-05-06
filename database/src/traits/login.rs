use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::{models::user::LoginModel, traits::database::Database};

#[async_trait]
pub trait Login: for<'long> Database<MySqlPool> {
    async fn login<'long>(
        body: LoginModel,
        database_connection: &'long Self::Connection,
    ) -> anyhow::Result<Self::Model>
    where
        Self: 'long,
        Self::Model: 'long,
        Self::Connection: 'long;

    async fn set_password(
        &mut self,
        password: String,
        connection_pool: &Self::Connection,
    ) -> anyhow::Result<()>;

    fn verify_password(password: &str, hash: &str) -> bool;
}
