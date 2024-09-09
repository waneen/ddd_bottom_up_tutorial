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
    TM: TransactionManager,
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
    TM: TransactionManager,
{
    let mut tx = tx_manager.lock().await.begin().await?;
    let res = usecase.register(&mut tx, info.name, info.email).await;
    match res {
        Ok(res) => {
            TM::commit(tx).await?;
            Ok(res)
        }
        Err(e) => {
            TM::rollback(tx).await?;
            Err(e.into())
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterUserRequestJdto {
    name: String,
    email: String,
}
