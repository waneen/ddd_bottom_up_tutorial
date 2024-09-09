use async_trait::async_trait;

use crate::{
    domain::{MailAddress, UserFactory, UserName},
    repository::{TransactionManager, UserRepository},
};

use super::{UserUseCaseImpl, UserUsecaseError};

// NOTE: traitとしてインターフェース化することで分業が可能
//       また、テストも可能になる。
#[async_trait]
pub trait UserRegisterUsecase<Tx>
where
    Tx: TransactionManager,
{
    async fn register(
        &self,
        tx: &mut Tx::Transaction<'_>,
        name: String,
        raw_mail_address: String,
    ) -> Result<(), UserUsecaseError>;
}

#[async_trait]
impl<Tx, Factory, Repo> UserRegisterUsecase<Tx> for UserUseCaseImpl<Tx, Factory, Repo>
where
    Tx: TransactionManager + std::marker::Sync + std::marker::Send,
    Repo: UserRepository<Tx> + std::marker::Sync,
    Factory: UserFactory + std::marker::Sync,
{
    async fn register(
        &self,
        tx: &mut Tx::Transaction<'_>,
        name: String,
        raw_mail_address: String,
    ) -> Result<(), UserUsecaseError> {
        // NOTE: トランザクションにより整合性が担保される
        //       →transactionを管理するものを作ってトランザクションを受け取る。
        // connection.begin_transaction();
        // MEMO: txを使用して解決
        let user = self
            .user_factory
            .create(UserName::new(name)?, MailAddress::new(raw_mail_address)?)?;

        // NOTE: domain_serviceで確認を行うことで変更に強い
        if self.user_service.exists(tx, &user).await? {
            return Err(UserUsecaseError::UserAlreadyExistsError(user.name));
        }

        self.user_repository.save(tx, user).await?;

        Ok(())
        // NOTE: トランザクションに問題がなければ永続化
        //       問題があれば、ロールバックする。
        // connection.commit()
    }
}

// NOTE: テスト用のモックを作成することでテストも容易になる
// pub struct MockUserRegisterUsecase<Repo, Factory>
// where
//     Repo: UserRepository,
//     Factory: UserFactory,
// {
//     user_factory: Factory,
//     user_repository: Repo,
//     user_service: UserService<Repo>,
//     // connection : Connection
// }

// impl<Repo, Factory> MockUserRegisterUsecase<Repo, Factory>
// where
//     Repo: UserRepository,
//     Factory: UserFactory,
// {
//     pub fn new(
//         user_factory: Factory,
//         user_repository: Repo,
//         user_service: UserService<Repo>,
//     ) -> Self {
//         Self {
//             user_factory,
//             user_repository,
//             user_service,
//         }
//     }
// }

// #[async_trait]
// impl<Repo, Factory> UserRegisterUsecase for MockUserRegisterUsecase<Repo, Factory>
// where
//     Repo: UserRepository + std::marker::Sync,
//     Factory: UserFactory + std::marker::Sync,
// {
//     type Error = UserUsecaseError<Repo::Error>;
//     async fn register(&self, name: String, raw_mail_address: String) -> Result<(), Self::Error> {
//         // NOTE: トランザクションにより整合性が担保される
//         // connection.begin_transaction();
//         let user = self
//             .user_factory
//             .create(UserName::new(name)?, MailAddress::new(raw_mail_address)?)?;

//         // NOTE: domain_serviceで確認を行うことで変更に強い
//         if self.user_service.exists(&user).await? {
//             return Err(UserUsecaseError::UserAlreadyExistsError(user.name));
//         }

//         self.user_repository.save(user).await?;

//         Ok(())
//         // NOTE: トランザクションに問題がなければ永続化
//         //       問題があれば、ロールバックする。
//         // connection.commit()
//     }
// }

// #[derive(Debug, thiserror::Error)]
// pub enum UserRegisterServiceError {
//     #[error(transparent)]
//     UserNameError(#[from] UserNameError),
//     #[error(transparent)]
//     MailAddressError(#[from] MailAddressError),
//     #[error(transparent)]
//     UserFactoryError(#[from] UserFactoryError),
//     #[error(transparent)]
//     UserRepositoryError(#[from] UserRepositoryError),
//     #[error(transparent)]
//     UserServiceError(#[from] UserServiceError),
//     #[error("{0}はすでに存在しています。")]
//     UserAlreadyExistsError(UserName),
// }
