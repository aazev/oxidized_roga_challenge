use async_trait::async_trait;

#[async_trait]
pub trait Database<T> {
    type Connection;
    type Model;

    async fn get<'long>(
        id: u64,
        database_connection: &'long Self::Connection,
    ) -> sqlx::Result<Self::Model>
    where
        Self: 'long,
        Self::Model: 'long,
        Self::Connection: 'long;

    async fn list<'long>(
        database_connection: &'long Self::Connection,
    ) -> sqlx::Result<Vec<Self::Model>>
    where
        Self: 'long,
        Self::Model: 'long,
        Self::Connection: 'long;
}
