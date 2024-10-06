use async_trait::async_trait;
use tokio::sync::Mutex;

use super::database_error::DatabaseError;

pub mod pg_transaction;

#[async_trait]
pub trait TransactionManager {
    type Transaction<'a>: 'a + std::marker::Send;

    async fn get_transaction<'a>(&self) -> Result<Self::Transaction<'a>, DatabaseError>;
    async fn commit(mut tx: Self::Transaction<'_>) -> Result<(), DatabaseError>;
    async fn rollback(mut tx: Self::Transaction<'_>) -> Result<(), DatabaseError>;
    async fn begin(tx_manager: &Mutex<Self>) -> Result<Self::Transaction<'_>, DatabaseError> {
        tx_manager.lock().await.get_transaction().await
    }
    async fn execute<T, UsecaseError, ControllerError>(
        tx: Self::Transaction<'_>,
        result: Result<T, UsecaseError>,
    ) -> Result<T, ControllerError>
    where
        T: Send,
        UsecaseError: Send,
        ControllerError: From<UsecaseError> + From<DatabaseError>,
    {
        match result {
            Ok(res) => {
                Self::commit(tx).await?;
                Ok(res)
            }
            Err(e) => {
                Self::rollback(tx).await?;
                Err(e.into())
            }
        }
    }
}
