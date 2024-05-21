use anyhow::Result;

use crate::{domain::User, repository::UserRepository};

pub struct UserService<Repo: UserRepository> {
    user_repository: Repo,
}

impl<Repo> UserService<Repo>
where
    Repo: UserRepository,
{
    pub fn new(user_repository: Repo) -> Self {
        Self { user_repository }
    }

    pub async fn exists(&self, user: &User) -> Result<bool> {
        let duplicated_user = self
            .user_repository
            .find_by_mail_address(&user.mail_address);
        Ok(duplicated_user.await?.is_some())
    }
}
