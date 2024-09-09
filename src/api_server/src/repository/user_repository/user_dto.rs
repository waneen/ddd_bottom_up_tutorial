use sqlx::FromRow;
use uuid::Uuid;

use crate::domain::{
    MailAddress, MailAddressError, User, UserId, UserIdError, UserName, UserNameError,
};

#[derive(FromRow)]
pub struct UserDto {
    pub user_id: Uuid,
    pub user_name: String,
    pub mail_address: String,
}

impl TryFrom<User> for UserDto {
    type Error = UserDomainToDtoConversionError;

    fn try_from(value: User) -> Result<Self, Self::Error> {
        Ok(Self {
            user_id: value.id.get(),
            user_name: value.name.into_inner(),
            mail_address: value.mail_address.into_inner(),
        })
    }
}

impl TryInto<User> for UserDto {
    type Error = UserDomainToDtoConversionError;

    fn try_into(self) -> Result<User, Self::Error> {
        Ok(User {
            id: UserId::new(self.user_id)?,
            name: UserName::new(self.user_name)?,
            mail_address: MailAddress::new(self.mail_address)?,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserDomainToDtoConversionError {
    #[error("Invalid UserId: {0}")]
    InvalidUserId(#[from] UserIdError),
    #[error("Invalid UserName: {0}")]
    InvalidUserName(#[from] UserNameError),
    #[error("Invalid MailAddress: {0}")]
    InvalidMailAddress(#[from] MailAddressError),
}
