use actix_web::web;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{repository::TransactionManager, use_case::UserRegisterUsecase};

use super::UserControllerError;

pub async fn handle_register_user<TM, Usecase>(
    info: web::Json<RegisterUserRequestJdto>,
    tx_manager: web::Data<Mutex<TM>>,
    usecase: web::Data<Usecase>,
) -> Result<web::Json<()>, actix_web::Error>
where
    Usecase: UserRegisterUsecase<TM>,
    TM: TransactionManager + Send,
{
    Ok(
        register_user_controller(tx_manager.as_ref(), usecase.as_ref(), info.into_inner())
            .await
            .map_err(|e| {
                println!("{e}");
                e
            })
            .map(web::Json)?,
    )
}

async fn register_user_controller<Usecase, TM>(
    tx_manager: &Mutex<TM>,
    usecase: &Usecase,
    info: RegisterUserRequestJdto,
) -> Result<(), UserControllerError>
where
    Usecase: UserRegisterUsecase<TM>,
    TM: TransactionManager + Send,
{
    let mut tx = TM::begin(tx_manager).await?;
    let res = usecase.register(&mut tx, info.name, info.email).await;
    TM::execute(tx, res).await
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterUserRequestJdto {
    name: String,
    email: String,
}
