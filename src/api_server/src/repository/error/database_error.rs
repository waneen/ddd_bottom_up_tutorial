#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}
