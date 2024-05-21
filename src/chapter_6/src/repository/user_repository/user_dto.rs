use sqlx::FromRow;
use uuid::Uuid;

use crate::domain::{MailAddress, User, UserId, UserName};

#[derive(FromRow)]
pub struct UserDto {
    pub user_id: Uuid,
    pub user_name: String,
    pub mail_address: String,
}

impl TryFrom<User> for UserDto {
    // TODO: エラーを適切に実装する
    type Error = anyhow::Error;

    fn try_from(value: User) -> Result<Self, Self::Error> {
        Ok(Self {
            user_id: value.id.get(),
            user_name: value.name.into_inner(),
            mail_address: value.mail_address.into_inner(),
        })
    }
}

impl TryInto<User> for UserDto {
    // TODO: エラーを適切に実装する
    type Error = anyhow::Error;

    fn try_into(self) -> Result<User, Self::Error> {
        Ok(User {
            id: UserId::new(self.user_id)?,
            name: UserName::new(self.user_name)?,
            mail_address: MailAddress::new(self.mail_address)?,
        })
    }
}
