use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    domain::{MailAddress, User, UserId, UserName},
    repository::user_repository::user_dto::UserDto,
};

use super::UserRepository;

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

// MEMO: UserDtoにTryFromとIntoを実装、new_type_patternをいい感じに処理してDBに入れるまで書く
#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<User>> {
        sqlx::query_as!(
            UserDto,
            "SELECT * FROM users WHERE user_id = $1",
            user_id.get()
        )
        .fetch_optional(&self.pool)
        .await?
        .map(TryInto::try_into)
        .transpose()
    }

    async fn find_by_user_name(&self, user_name: &UserName) -> Result<Option<User>> {
        sqlx::query_as!(
            UserDto,
            "SELECT * FROM users WHERE user_name = $1",
            user_name.get(),
        )
        .fetch_optional(&self.pool)
        .await?
        .map(TryInto::try_into)
        .transpose()
    }

    async fn find_by_mail_address(&self, mail_address: &MailAddress) -> Result<Option<User>> {
        sqlx::query_as!(
            UserDto,
            "SELECT * FROM users WHERE mail_address = $1",
            mail_address.get(),
        )
        .fetch_optional(&self.pool)
        .await?
        .map(TryInto::try_into)
        .transpose()
    }

    async fn save(&self, user: User) -> Result<()> {
        sqlx::query!(
            "INSERT INTO users (user_id, user_name, mail_address) VALUES ($1, $2, $3) ON CONFLICT (user_id) DO UPDATE SET user_name = $2, mail_address = $3",
            user.id.get(),
            user.name.get(),
            user.mail_address.get(),
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, user: User) -> Result<()> {
        sqlx::query!("DELETE FROM users WHERE user_id = $1", user.id.get())
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
