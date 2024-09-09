mod default_user_factory;
pub use default_user_factory::DefaultUserFactory;

use crate::domain::{MailAddress, User, UserIdError, UserName};

pub trait UserFactory {
    fn create(&self, name: UserName, mail_address: MailAddress) -> Result<User, UserFactoryError>;
}

pub trait HasUserFactory {
    type UserFactory: UserFactory;
    fn user_repository(&self) -> &Self::UserFactory;
}

#[derive(Debug, thiserror::Error)]
pub enum UserFactoryError {
    #[error(transparent)]
    UserIdError(#[from] UserIdError),
}
