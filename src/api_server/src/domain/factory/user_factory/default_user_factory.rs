use uuid::Uuid;

use crate::domain::{MailAddress, User, UserId, UserName};

use super::{UserFactory, UserFactoryError};

#[derive(Default, Clone)]
pub struct DefaultUserFactory {}

impl UserFactory for DefaultUserFactory {
    fn create(&self, name: UserName, mail_address: MailAddress) -> Result<User, UserFactoryError> {
        Ok(User::new(UserId::new(Uuid::new_v4())?, name, mail_address))
    }
}
