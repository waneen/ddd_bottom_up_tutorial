use anyhow::Result;

mod default_user_factory;
pub use default_user_factory::DefaultUserFactory;

use crate::domain::{MailAddress, User, UserName};

pub trait UserFactory {
    fn create(&self, name: UserName, mail_address: MailAddress) -> Result<User>;
}
