use anyhow::Result;
use uuid::Uuid;

use crate::domain::{MailAddress, User, UserId, UserName};

use super::UserFactory;

pub struct DefaultUserFactory {}

impl UserFactory for DefaultUserFactory {
    fn create(&self, name: UserName, mail_address: MailAddress) -> Result<User> {
        Ok(User::new(UserId::new(Uuid::new_v4()), name, mail_address))
    }
}
