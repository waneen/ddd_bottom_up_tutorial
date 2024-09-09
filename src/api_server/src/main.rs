use std::sync::Arc;

use actix_web::{App, HttpServer};
use env_logger::Env;

use api_server::*;
use clap::Parser as _;
use repository::pg_transaction::PgTransactionManager;
use tokio::sync::Mutex;

#[derive(Debug, clap::Parser)]
#[command(version,about,long_about=None)]
pub struct ApiServerArguments {
    /// database user
    #[arg(long, env("DB_USER"))]
    database_user: String,
    /// database password
    #[arg(long, env("DB_PASSWORD"))]
    database_password: String,
    /// database host
    #[arg(long, env("DB_HOST"))]
    database_host: String,
    /// database port
    #[arg(long, env("DB_PORT"))]
    database_port: u16,
    /// database name
    #[arg(long, env("DB_NAME"))]
    database_name: String,
}

impl ApiServerArguments {
    fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.database_user,
            self.database_password,
            self.database_host,
            self.database_port,
            self.database_name,
        )
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 環境変数からDBの接続情報を取得
    let args = ApiServerArguments::parse();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // DB接続プールの作成
    let pool = Arc::new(
        sqlx::PgPool::connect(&args.database_url())
            .await
            .expect("database connection failed"),
    );

    let tm = Arc::new(Mutex::new(PgTransactionManager::new(pool)));

    // リポジトリの作成
    let user_repository = repository::PgUserRepository {};

    let user_factory = domain::DefaultUserFactory::default();

    // サービスの作成
    let user_service = domain::UserService::new(user_repository.clone());
    let user_usecase = Arc::new(use_case::UserUseCaseImpl::new(
        user_factory,
        user_repository.clone(),
        user_service,
    ));

    // Actix Web アプリケーションの起動
    HttpServer::new(move || {
        App::new().configure(|cfg| {
            controller::user_controller::config(cfg, user_usecase.clone(), tm.clone())
        })
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
