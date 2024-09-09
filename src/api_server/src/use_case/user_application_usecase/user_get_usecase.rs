use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domain::{UserFactory, UserId},
    repository::{TransactionManager, UserRepository},
};

use super::{UserDto, UserUseCaseImpl, UserUsecaseError};

#[async_trait]
pub trait UserGetUsecase<Tx>
where
    Tx: TransactionManager,
{
    async fn get(
        &self,
        tx: &mut Tx::Transaction<'_>,
        user_id: &Uuid,
    ) -> Result<Option<UserDto>, UserUsecaseError>;
}

#[async_trait]
impl<Tx, Factory, Repo> UserGetUsecase<Tx> for UserUseCaseImpl<Tx, Factory, Repo>
where
    Tx: TransactionManager + std::marker::Sync + std::marker::Send,
    Repo: UserRepository<Tx> + std::marker::Sync,
    Factory: UserFactory + std::marker::Sync,
{
    // NOTE: DTOを用いることで、ドメインの流出を防ぐことができる
    async fn get(
        &self,
        tx: &mut Tx::Transaction<'_>,
        user_id: &Uuid,
    ) -> Result<Option<UserDto>, UserUsecaseError> {
        let target_id = UserId::new(*user_id)?;
        Ok(self
            .user_repository
            .find_by_user_id(tx, &target_id)
            .await?
            .map(|user| UserDto {
                user_id: user.id.get(),
                user_name: user.name.into_inner(),
                mail_address: user.mail_address.into_inner(),
            }))
    }
}

// pub struct MockUserGetUsecase<Repo>
// where
//     Repo: UserRepository,
// {
//     user_repository: Repo,
// }

// impl<Repo> MockUserGetUsecase<Repo>
// where
//     Repo: UserRepository,
// {
//     pub fn new(user_repository: Repo) -> Self {
//         Self { user_repository }
//     }
// }

// #[derive(Debug, thiserror::Error)]
// pub enum UserGetServiceError {
//     #[error(transparent)]
//     UserIdError(#[from] UserIdError),
//     #[error(transparent)]
//     UserRepositoryError(#[from] UserRepositoryError),
// }
