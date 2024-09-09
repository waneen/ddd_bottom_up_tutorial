use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domain::{UserFactory, UserId},
    repository::{TransactionManager, UserRepository},
};

use super::{UserUseCaseImpl, UserUsecaseError};

#[async_trait]
pub trait UserDeleteUsecase<Tx>
where
    Tx: TransactionManager,
{
    async fn delete(
        &self,
        tx: &mut Tx::Transaction<'_>,
        user_id: Uuid,
    ) -> Result<(), UserUsecaseError>;
}

#[async_trait]
impl<Tx, Factory, Repo> UserDeleteUsecase<Tx> for UserUseCaseImpl<Tx, Factory, Repo>
where
    Tx: TransactionManager + std::marker::Sync + std::marker::Send,
    Repo: UserRepository<Tx> + std::marker::Sync,
    Factory: UserFactory + std::marker::Sync,
{
    async fn delete(
        &self,
        tx: &mut Tx::Transaction<'_>,
        user_id: Uuid,
    ) -> Result<(), UserUsecaseError> {
        let target_id = UserId::new(user_id)?;
        // NOTE: Userが見つからなかった場合も退会成功とする場合もある
        let target_user = self
            .user_repository
            .find_by_user_id(tx, &target_id)
            .await?
            .ok_or_else(|| UserUsecaseError::UserIdNotExistsError(target_id))?;

        Ok(self.user_repository.delete(tx, target_user).await?)
    }
}

// pub struct MockUserDeleteUsecase<TM, Repo>
// where
//     TM: TransactionManager,
//     Repo: UserRepository<TM>,
// {
//     user_repository: Repo,
// }

// impl<TM, Repo> MockUserDeleteUsecase<TM, Repo>
// where
//     TM: TransactionManager,
//     Repo: UserRepository<TM>,
// {
//     pub fn new(user_repository: Repo) -> Self {
//         Self { user_repository }
//     }
// }

// #[derive(Debug, thiserror::Error)]
// pub enum UserDeleteServiceError {
//     #[error(transparent)]
//     UserIdError(#[from] UserIdError),
//     #[error(transparent)]
//     UserRepositoryError(#[from] UserRepositoryError),
//     #[error("{0}は不適切なuser_idです")]
//     UserIdNotExistsError(UserId),
// }
