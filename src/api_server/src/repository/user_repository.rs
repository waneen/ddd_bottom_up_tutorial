use async_trait::async_trait;
use user_dto::UserDomainToDtoConversionError;

use crate::domain::{MailAddress, User, UserId, UserName};

mod pg_user_repository;
mod user_dto;
pub use pg_user_repository::PgUserRepository;

use super::{database_error::DatabaseError, TransactionManager};

#[async_trait]
pub trait UserRepository<TM>
where
    TM: TransactionManager,
{
    // NOTE: 検索クエリを導入してfindやupdateをUserQuery型を受け取るように変更する？
    async fn find_by_user_id(
        &self,
        tx: &mut TM::Transaction<'_>,
        user_id: &UserId,
    ) -> Result<Option<User>, UserRepositoryError>;
    async fn find_by_user_name(
        &self,
        tx: &mut TM::Transaction<'_>,
        user_name: &UserName,
    ) -> Result<Option<User>, UserRepositoryError>;
    async fn find_by_mail_address(
        &self,
        tx: &mut TM::Transaction<'_>,
        mail_address: &MailAddress,
    ) -> Result<Option<User>, UserRepositoryError>;
    async fn save(
        &self,
        tx: &mut TM::Transaction<'_>,
        user: User,
    ) -> Result<(), UserRepositoryError>;
    async fn delete(
        &self,
        tx: &mut TM::Transaction<'_>,
        user: User,
    ) -> Result<(), UserRepositoryError>;
}

#[derive(Debug, thiserror::Error)]
pub enum UserRepositoryError {
    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
    #[error(transparent)]
    ConversionError(#[from] UserDomainToDtoConversionError),
}
