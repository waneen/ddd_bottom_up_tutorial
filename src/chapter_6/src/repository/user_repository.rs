use anyhow::Result;
use async_trait::async_trait;

use crate::domain::{MailAddress, User, UserId, UserName};

mod pg_user_repository;
mod user_dto;
pub use pg_user_repository::PgUserRepository;

#[async_trait]
pub trait UserRepository {
    // NOTE: 検索クエリを導入してfindやupdateをUserQuery型を受け取るように変更する？
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<User>>;
    async fn find_by_user_name(&self, user_name: &UserName) -> Result<Option<User>>;
    async fn find_by_mail_address(&self, mail_address: &MailAddress) -> Result<Option<User>>;
    async fn save(&self, user: User) -> Result<()>;
    async fn delete(&self, user: User) -> Result<()>;
}
