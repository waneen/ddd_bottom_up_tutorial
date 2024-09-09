use async_trait::async_trait;

use super::database_error::DatabaseError;

pub mod pg_transaction;

#[async_trait]
pub trait TransactionManager {
    type Transaction<'a>: 'a + std::marker::Send;

    async fn begin<'a>(&self) -> Result<Self::Transaction<'a>, DatabaseError>;
    async fn commit(mut tx: Self::Transaction<'_>) -> Result<(), DatabaseError>;
    async fn rollback(mut tx: Self::Transaction<'_>) -> Result<(), DatabaseError>;
}
