use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

use crate::{
    domain::{MailAddress, User, UserId, UserName},
    repository::{
        database_error::DatabaseError, pg_transaction::PgTransactionManager,
        user_repository::user_dto::UserDto,
    },
};

use super::{UserRepository, UserRepositoryError};

#[derive(Clone)]
pub struct PgUserRepository {}

impl PgUserRepository {}

#[async_trait]
impl UserRepository<PgTransactionManager> for PgUserRepository {
    async fn find_by_user_id(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_id: &UserId,
    ) -> Result<Option<User>, UserRepositoryError> {
        let user_dto = sqlx::query_as!(
            UserDto,
            "SELECT * FROM users WHERE user_id = $1",
            user_id.get()
        )
        .fetch_optional(&mut **tx)
        .await
        .map_err(DatabaseError::from)?;
        user_dto
            .map(|user_dto| Ok(user_dto.try_into()?))
            .transpose()
    }

    async fn find_by_user_name(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_name: &UserName,
    ) -> Result<Option<User>, UserRepositoryError> {
        let user_dto = sqlx::query_as!(
            UserDto,
            "SELECT * FROM users WHERE user_name = $1",
            user_name.get(),
        )
        .fetch_optional(&mut **tx)
        .await
        .map_err(DatabaseError::from)?;
        user_dto
            .map(|user_dto| Ok(user_dto.try_into()?))
            .transpose()
    }

    async fn find_by_mail_address(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        mail_address: &MailAddress,
    ) -> Result<Option<User>, UserRepositoryError> {
        let user_dto = sqlx::query_as!(
            UserDto,
            "SELECT * FROM users WHERE mail_address = $1",
            mail_address.get(),
        )
        .fetch_optional(&mut **tx)
        .await
        .map_err(DatabaseError::from)?;
        user_dto
            .map(|user_dto| Ok(user_dto.try_into()?))
            .transpose()
    }

    async fn save(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user: User,
    ) -> Result<(), UserRepositoryError> {
        sqlx::query!(
            "INSERT INTO users (user_id, user_name, mail_address) VALUES ($1, $2, $3) ON CONFLICT (user_id) DO UPDATE SET user_name = $2, mail_address = $3",
            user.id.get(),
            user.name.get(),
            user.mail_address.get(),
        )
        .execute(&mut **tx)
        .await.map_err(DatabaseError::from)?;
        Ok(())
    }

    async fn delete(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user: User,
    ) -> Result<(), UserRepositoryError> {
        sqlx::query!("DELETE FROM users WHERE user_id = $1", user.id.get())
            .execute(&mut **tx)
            .await
            .map_err(DatabaseError::from)?;
        Ok(())
    }
}
