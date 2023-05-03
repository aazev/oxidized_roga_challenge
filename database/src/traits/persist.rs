use crate::traits::database::Database;
use async_trait::async_trait;
use sqlx::{error::BoxDynError, MySqlPool};

#[async_trait]
pub trait Persist: for<'long> Database<MySqlPool> {
    async fn insert<'long>(
        &'long self,
        database_connection: &'long MySqlPool,
    ) -> sqlx::Result<Self::Model>
    where
        Self: 'long,
        Self::Model: 'long;

    async fn update<'long>(
        &'long self,
        database_connection: &'long MySqlPool,
    ) -> sqlx::Result<Self::Model>
    where
        Self: 'long,
        Self::Model: 'long;

    async fn delete<'long>(
        &'long self,
        database_connection: &'long MySqlPool,
    ) -> Result<(), BoxDynError>
    where
        Self: 'long;
}
