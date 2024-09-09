use crate::{
    domain::User,
    repository::{TransactionManager, UserRepository, UserRepositoryError},
};

#[derive(Clone)]
pub struct UserService<TM, Repo> {
    _marker: std::marker::PhantomData<fn() -> TM>,
    user_repository: Repo,
}

impl<TM, Repo> UserService<TM, Repo>
where
    TM: TransactionManager,
    Repo: UserRepository<TM>,
{
    pub fn new(user_repository: Repo) -> Self {
        Self {
            _marker: std::marker::PhantomData,
            user_repository,
        }
    }

    pub async fn exists(
        &self,
        tx: &mut TM::Transaction<'_>,
        user: &User,
    ) -> Result<bool, UserServiceError> {
        let duplicated_user = self
            .user_repository
            .find_by_mail_address(tx, &user.mail_address)
            .await?;
        Ok(duplicated_user.is_some())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserServiceError {
    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),
}
