use crate::domain::{
    value_object::MailAddress, MailAddressError, UserId, UserIdError, UserName, UserNameError,
};

pub struct User {
    pub id: UserId,
    pub name: UserName,
    pub mail_address: MailAddress,
}

impl User {
    pub fn new(id: UserId, name: UserName, mail_address: MailAddress) -> Self {
        Self {
            id,
            name,
            mail_address,
        }
    }

    // NOTE: factoryを作ったことによりこちらがnewになる
    // pub fn new_with_id(id: UserId, name: UserName, mail_address: MailAddress) -> Self {
    //     Self {
    //         id,
    //         name,
    //         mail_address,
    //     }
    // }

    pub fn change_name(&mut self, name: UserName) {
        self.name = name;
    }

    pub fn change_mail_address(&mut self, mail_address: MailAddress) {
        self.mail_address = mail_address;
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct UserUpdateCommand {
    pub name: Option<String>,
    pub mail_address: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error(transparent)]
    UserIdError(#[from] UserIdError),
    #[error(transparent)]
    UserNameError(#[from] UserNameError),
    #[error(transparent)]
    MailAddressError(#[from] MailAddressError),
}
