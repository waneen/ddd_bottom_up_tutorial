use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};

use crate::repository::database_error::DatabaseError;

use super::TransactionManager;

pub struct PgTransactionManager {
    pool: Arc<PgPool>,
}

impl PgTransactionManager {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TransactionManager for PgTransactionManager {
    type Transaction<'a> = Transaction<'a, Postgres>;

    async fn get_transaction<'a>(&self) -> Result<Self::Transaction<'a>, DatabaseError> {
        Ok(self.pool.begin().await?)
    }

    async fn commit(tx: Self::Transaction<'_>) -> Result<(), DatabaseError> {
        Ok(tx.commit().await?)
    }

    async fn rollback(tx: Self::Transaction<'_>) -> Result<(), DatabaseError> {
        Ok(tx.rollback().await?)
    }
}
