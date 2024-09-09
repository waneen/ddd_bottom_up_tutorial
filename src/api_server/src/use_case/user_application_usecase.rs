mod user_delete_usecase;
mod user_dto;
mod user_get_usecase;
mod user_register_usecase;
mod user_update_usecase;

pub use user_delete_usecase::*;
pub use user_dto::UserDto;
pub use user_get_usecase::*;
pub use user_register_usecase::*;
pub use user_update_usecase::*;

use crate::{
    domain::{
        MailAddressError, UserFactoryError, UserId, UserIdError, UserName, UserNameError,
        UserService, UserServiceError,
    },
    repository::UserRepositoryError,
};

pub struct UserUseCaseImpl<Tx, Factory, Repo> {
    user_factory: Factory,
    user_repository: Repo,
    user_service: UserService<Tx, Repo>,
}

impl<Tx, Factory, Repo> UserUseCaseImpl<Tx, Factory, Repo> {
    pub fn new(
        user_factory: Factory,
        user_repository: Repo,
        user_service: UserService<Tx, Repo>,
    ) -> Self {
        Self {
            user_factory,
            user_repository,
            user_service,
        }
    }
}

pub trait TUserUsecaseError: std::error::Error + std::marker::Send + std::marker::Sync {}

#[derive(Debug, thiserror::Error)]
pub enum UserUsecaseError {
    #[error(transparent)]
    UserIdError(#[from] UserIdError),
    #[error(transparent)]
    UserNameError(#[from] UserNameError),
    #[error(transparent)]
    MailAddressError(#[from] MailAddressError),
    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),
    #[error(transparent)]
    UserServiceError(#[from] UserServiceError),
    #[error(transparent)]
    UserFactoryError(#[from] UserFactoryError),
    #[error("{0}はすでに存在しています。")]
    UserAlreadyExistsError(UserName),
    #[error("{0}は不適切なuser_idです")]
    UserIdNotExistsError(UserId),
}
