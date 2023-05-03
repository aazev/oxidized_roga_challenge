use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::{models::user::LoginModel, traits::database::Database};

#[async_trait]
pub trait Login: for<'long> Database<MySqlPool> {
    async fn login<'long>(
        body: LoginModel,
        database_connection: &'long Self::Connection,
    ) -> sqlx::Result<Self::Model>
    where
        Self: 'long,
        Self::Model: 'long,
        Self::Connection: 'long;
}
