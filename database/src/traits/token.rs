use async_trait::async_trait;
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::traits::database::Database;

#[async_trait]
pub trait Token: for<'long> Database<MySqlPool> {
    async fn get_by_uuid<'long>(
        uuid: Uuid,
        database_connection: &'long Self::Connection,
    ) -> sqlx::Result<Self::Model>
    where
        Self: 'long,
        Self::Model: 'long,
        Self::Connection: 'long;
}
