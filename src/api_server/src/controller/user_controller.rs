use actix_web::{web, HttpResponse};
use std::sync::Arc;
use tokio::sync::Mutex;

mod register;
mod update;

use register::*;
use update::*;

use crate::{
    repository::{database_error::DatabaseError, TransactionManager},
    use_case::{UserRegisterUsecase, UserUpdateUsecase, UserUsecaseError},
};

pub fn config<TM, Usecase>(cfg: &mut web::ServiceConfig, usecase: Arc<Usecase>, tm: Arc<Mutex<TM>>)
where
    TM: TransactionManager + std::marker::Sync + std::marker::Send + 'static,
    Usecase: UserRegisterUsecase<TM>
        + UserUpdateUsecase<TM>
        + std::marker::Send
        + std::marker::Sync
        + 'static,
{
    let usecase_data = web::Data::from(usecase);
    let tm_data = web::Data::from(tm);
    cfg.app_data(tm_data)
        .app_data(usecase_data)
        .route(
            "/users",
            web::post().to(handle_register_user::<TM, Usecase>),
        )
        .route("/users/{id}", web::put().to(update_user::<TM, Usecase>));
}

#[derive(Debug, thiserror::Error)]
pub enum UserControllerError {
    #[error(transparent)]
    UserApplicationError(#[from] UserUsecaseError),
    #[error("DatabaseConnectionError")]
    DatabaseError(#[from] DatabaseError),
}

impl actix_web::ResponseError for UserControllerError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            // TODO: 適切にハンドリング
            Self::UserApplicationError(_) => actix_web::http::StatusCode::UNPROCESSABLE_ENTITY,
            Self::DatabaseError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            // TODO: 適切にハンドリング
            Self::UserApplicationError(_) => {
                HttpResponse::InternalServerError().body(self.to_string())
            }
            Self::DatabaseError(_) => HttpResponse::InternalServerError().body(self.to_string()),
        }
    }
}
