use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domain::{MailAddress, UserId, UserName, UserUpdateCommand},
    repository::{TransactionManager, UserRepository},
};

use super::{UserUseCaseImpl, UserUsecaseError};

#[async_trait(?Send)]
pub trait UserUpdateUsecase<Tx>
where
    Tx: TransactionManager,
{
    async fn update(
        &self,
        tx: &mut Tx::Transaction<'_>,
        user_id: Uuid,
        user_update_command: UserUpdateCommand,
    ) -> Result<(), UserUsecaseError>;
}

#[async_trait(?Send)]
impl<Tx, Factory, Repo> UserUpdateUsecase<Tx> for UserUseCaseImpl<Tx, Factory, Repo>
where
    Tx: TransactionManager,
    Repo: UserRepository<Tx> + std::marker::Sync,
{
    async fn update(
        &self,
        tx: &mut Tx::Transaction<'_>,
        user_id: Uuid,
        user_update_command: UserUpdateCommand,
    ) -> Result<(), UserUsecaseError> {
        let target_id = UserId::new(user_id)?;
        let mut target_user = self
            .user_repository
            .find_by_user_id(tx, &target_id)
            .await?
            .ok_or_else(|| UserUsecaseError::UserIdNotExistsError(target_id))?;

        if let Some(new_user_name) = user_update_command.name {
            // if self.user_service.exists(tx, &target_user).await? {
            //     return Err(UserUsecaseError::UserAlreadyExistsError(target_user.name));
            // }

            target_user.change_name(UserName::new(new_user_name)?);
        }

        if let Some(new_mail_address) = user_update_command.mail_address {
            target_user.change_mail_address(MailAddress::new(new_mail_address)?);
        }

        Ok(self.user_repository.save(tx, target_user).await?)
    }
}

// pub struct MockUserUpdateUsecase<Repo>
// where
//     Repo: UserRepository,
// {
//     user_repository: Repo,
//     user_service: UserService<Repo>,
// }

// impl<Repo> MockUserUpdateUsecase<Repo>
// where
//     Repo: UserRepository,
// {
//     pub fn new(user_repository: Repo, user_service: UserService<Repo>) -> Self {
//         Self {
//             user_repository,
//             user_service,
//         }
//     }
// }

// #[async_trait(?Send)]
// impl<Repo> UserUpdateUsecase for MockUserUpdateUsecase<Repo>
// where
//     Repo: UserRepository + std::marker::Sync + std::marker::Send,
// {
//     type Error = UserUsecaseError<Repo::Error>;
//     // NOTE: command_objectを使用することで、シグネチャを変更することなく実装可能
//     //       そもそもuse_caseとしてまとめて変更するか、1項目ずつ変更するかも重要
//     async fn update(
//         &self,
//         user_id: Uuid,
//         user_update_command: UserUpdateCommand,
//     ) -> Result<(), Self::Error> {
//         let target_id = UserId::new(user_id)?;
//         let mut target_user = self
//             .user_repository
//             .find_by_user_id(&target_id)
//             .await?
//             .ok_or_else(|| UserUsecaseError::UserIdNotExistsError(target_id))?;

//         if let Some(new_user_name) = user_update_command.name {
//             target_user.change_name(UserName::new(new_user_name)?);

//             if self.user_service.exists(&target_user).await? {
//                 return Err(UserUsecaseError::UserAlreadyExistsError(target_user.name));
//             }
//         }

//         if let Some(new_mail_address) = user_update_command.mali_address {
//             target_user.change_mail_address(MailAddress::new(new_mail_address)?);
//         }

//         Ok(self.user_repository.save(target_user).await?)
//     }
// }

// #[derive(Debug, thiserror::Error)]
// pub enum UserUpdateServiceError {
//     #[error(transparent)]
//     UserIdError(#[from] UserIdError),
//     #[error(transparent)]
//     UserRepositoryError(#[from] UserRepositoryError),
//     #[error(transparent)]
//     UserServiceError(#[from] UserServiceError),
//     #[error("{0}はすでに存在しています。")]
//     UserAlreadyExistsError(UserName),
//     #[error("{0}は不適切なuser_idです")]
//     UserIdNotExistsError(UserId),
// }
