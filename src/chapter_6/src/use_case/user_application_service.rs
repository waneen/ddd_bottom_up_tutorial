use anyhow::{anyhow, Context, Result};
use uuid::Uuid;

use crate::{
    domain::{MailAddress, UserFactory, UserId, UserName, UserService, UserUpdateCommand},
    dto::UserDto,
    repository::UserRepository,
};

// NOTE: traitとしてインターフェース化することで分業が可能
//       また、テストも可能になる。
pub trait UserRegisterService {
    fn register(&self, name: String, raw_mail_address: String) -> Result<()>;
}

// NOTE: テスト用のモックを作成することでテストも容易になる
pub struct MockUserRegisterService<Repo: UserRepository, Factory: UserFactory> {
    user_factory: Factory,
    user_repository: Repo,
    user_service: UserService<Repo>,
    // connection : Connection
}

impl<Repo, Factory> UserRegisterService for MockUserRegisterService<Repo, Factory>
where
    Repo: UserRepository,
    Factory: UserFactory,
{
    fn register(&self, name: String, raw_mail_address: String) -> Result<()> {
        // NOTE: トランザクションにより整合性が担保される
        // connection.begin_transaction();
        let user = self
            .user_factory
            .create(UserName::new(name)?, MailAddress::new(raw_mail_address)?)?;

        // NOTE: domain_serviceで確認を行うことで変更に強い
        if self.user_service.exists(&user) {
            return Err(anyhow!("{}は既に存在しています。", user));
        }

        self.user_repository.save(user)?;

        Ok(())
        // NOTE: トランザクションに問題がなければ永続化
        //       問題があれば、ロールバックする。
        // connection.commit()
    }
}

pub trait UserUpdateService {
    fn update(&self, user_id: Uuid, user_update_command: UserUpdateCommand) -> Result<()>;
}

pub struct MockUserUpdateService<Repo: UserRepository> {
    user_repository: Repo,
    user_service: UserService<Repo>,
}

impl<Repo> UserUpdateService for MockUserUpdateService<Repo>
where
    Repo: UserRepository,
{
    // NOTE: command_objectを使用することで、シグネチャを変更することなく実装可能
    //       そもそもuse_caseとしてまとめて変更するか、1項目ずつ変更するかも重要
    fn update(&self, user_id: Uuid, user_update_command: UserUpdateCommand) -> Result<()> {
        let target_id = UserId::new(user_id);
        let mut target_user = self
            .user_repository
            .find_by_user_id(&target_id)
            .with_context(|| format!("{}は不適切なuser_idです", user_id))?;

        if let Some(new_user_name) = user_update_command.name {
            target_user.change_name(new_user_name);

            if self.user_service.exists(&target_user) {
                return Err(anyhow!("ユーザーは既に存在しています。"));
            }
        }

        if let Some(new_mail_address) = user_update_command.mali_address {
            target_user.change_mail_address(new_mail_address);
        }

        self.user_repository.save(target_user)
    }
}

pub trait UserDeleteService {
    fn delete(&self, user_id: Uuid) -> Result<()>;
}

pub struct MockUserDeleteService<Repo: UserRepository> {
    user_repository: Repo,
}

impl<Repo> UserDeleteService for MockUserDeleteService<Repo>
where
    Repo: UserRepository,
{
    fn delete(&self, user_id: Uuid) -> Result<()> {
        let target_id = UserId::new(user_id);
        // NOTE: Userが見つからなかった場合も退会成功とする場合もある
        let target_user = self
            .user_repository
            .find_by_user_id(&target_id)
            .with_context(|| format!("{}は不適切なuser_idです", user_id))?;

        self.user_repository.delete(target_user)
    }
}

pub trait UserGetService {
    fn get(&self, user_id: &Uuid) -> Option<UserDto>;
}

pub struct MockUserGetService<Repo: UserRepository> {
    user_repository: Repo,
}

impl<Repo> UserGetService for MockUserGetService<Repo>
where
    Repo: UserRepository,
{
    // NOTE: DTOを用いることで、ドメインの流出を防ぐことができる
    fn get(&self, user_id: &Uuid) -> Option<UserDto> {
        let target_id = UserId::new(*user_id);
        self.user_repository
            .find_by_user_id(&target_id)
            .map(|user| user.into())
    }
}
