use uuid::Uuid;

pub struct UserDto {
    pub user_id: Uuid,
    pub user_name: String,
    pub mail_address: String,
}

// NOTE: Dto⇔domainの変換ロジックはサービス層に書く。
// impl From<User> for UserDto {
//     fn from(value: User) -> Self {
//         Self {
//             id: value.id.0.to_string(),
//             name: value.name.get(),
//             mail_address: value.mail_address.0,
//         }
//     }
// }
