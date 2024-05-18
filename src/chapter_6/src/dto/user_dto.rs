use crate::domain::User;

pub struct UserDto {
    pub id: String,
    pub name: String,
    pub mail_address: String,
}

impl From<User> for UserDto {
    fn from(value: User) -> Self {
        Self {
            id: value.id.0.to_string(),
            name: value.name.get(),
            mail_address: value.mail_address.0,
        }
    }
}
