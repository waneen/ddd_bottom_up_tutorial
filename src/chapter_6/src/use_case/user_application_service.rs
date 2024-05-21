use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domain::{MailAddress, UserFactory, UserId, UserName, UserService, UserUpdateCommand},
    dto::UserDto,
    repository::UserRepository,
};

// NOTE: traitとしてインターフェース化することで分業が可能
//       また、テストも可能になる。
#[async_trait]
pub trait UserRegisterService {
    async fn register(&self, name: String, raw_mail_address: String) -> Result<()>;
}

// NOTE: テスト用のモックを作成することでテストも容易になる
pub struct MockUserRegisterService<Repo: UserRepository, Factory: UserFactory> {
    user_factory: Factory,
    user_repository: Repo,
    user_service: UserService<Repo>,
    // connection : Connection
}

#[async_trait]
impl<Repo, Factory> UserRegisterService for MockUserRegisterService<Repo, Factory>
where
    Repo: UserRepository + std::marker::Sync,
    Factory: UserFactory + std::marker::Sync,
{
    async fn register(&self, name: String, raw_mail_address: String) -> Result<()> {
        // NOTE: トランザクションにより整合性が担保される
        // connection.begin_transaction();
        let user = self
            .user_factory
            .create(UserName::new(name)?, MailAddress::new(raw_mail_address)?)?;

        // NOTE: domain_serviceで確認を行うことで変更に強い
        if self.user_service.exists(&user).await? {
            return Err(anyhow!("{}は既に存在しています。", user));
        }

        self.user_repository.save(user).await?;

        Ok(())
        // NOTE: トランザクションに問題がなければ永続化
        //       問題があれば、ロールバックする。
        // connection.commit()
    }
}

#[async_trait]
pub trait UserUpdateService {
    async fn update(&self, user_id: Uuid, user_update_command: UserUpdateCommand) -> Result<()>;
}

pub struct MockUserUpdateService<Repo: UserRepository> {
    user_repository: Repo,
    user_service: UserService<Repo>,
}

#[async_trait]
impl<Repo> UserUpdateService for MockUserUpdateService<Repo>
where
    Repo: UserRepository + std::marker::Sync,
{
    // NOTE: command_objectを使用することで、シグネチャを変更することなく実装可能
    //       そもそもuse_caseとしてまとめて変更するか、1項目ずつ変更するかも重要
    async fn update(&self, user_id: Uuid, user_update_command: UserUpdateCommand) -> Result<()> {
        let target_id = UserId::new(user_id)?;
        let mut target_user = self
            .user_repository
            .find_by_user_id(&target_id)
            .await?
            .with_context(|| format!("{}は不適切なuser_idです", user_id))?;

        if let Some(new_user_name) = user_update_command.name {
            target_user.change_name(new_user_name);

            if self.user_service.exists(&target_user).await? {
                return Err(anyhow!("ユーザーは既に存在しています。"));
            }
        }

        if let Some(new_mail_address) = user_update_command.mali_address {
            target_user.change_mail_address(new_mail_address);
        }

        self.user_repository.save(target_user).await
    }
}

#[async_trait]
pub trait UserDeleteService {
    async fn delete(&self, user_id: Uuid) -> Result<()>;
}

pub struct MockUserDeleteService<Repo: UserRepository> {
    user_repository: Repo,
}

#[async_trait]
impl<Repo> UserDeleteService for MockUserDeleteService<Repo>
where
    Repo: UserRepository + std::marker::Sync,
{
    async fn delete(&self, user_id: Uuid) -> Result<()> {
        let target_id = UserId::new(user_id)?;
        // NOTE: Userが見つからなかった場合も退会成功とする場合もある
        let target_user = self
            .user_repository
            .find_by_user_id(&target_id)
            .await?
            .with_context(|| format!("{}は不適切なuser_idです", user_id))?;

        self.user_repository.delete(target_user).await
    }
}

#[async_trait]
pub trait UserGetService {
    async fn get(&self, user_id: &Uuid) -> Result<Option<UserDto>>;
}

pub struct MockUserGetService<Repo: UserRepository> {
    user_repository: Repo,
}

#[async_trait]
impl<Repo> UserGetService for MockUserGetService<Repo>
where
    Repo: UserRepository + std::marker::Sync,
{
    // NOTE: DTOを用いることで、ドメインの流出を防ぐことができる
    async fn get(&self, user_id: &Uuid) -> Result<Option<UserDto>> {
        let target_id = UserId::new(*user_id)?;
        Ok(self
            .user_repository
            .find_by_user_id(&target_id)
            .await?
            .map(|user| UserDto {
                user_id: user.id.get(),
                user_name: user.name.into_inner(),
                mail_address: user.mail_address.into_inner(),
            }))
    }
}
