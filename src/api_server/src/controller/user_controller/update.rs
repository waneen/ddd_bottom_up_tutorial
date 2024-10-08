use actix_web::web;
use serde::Deserialize;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
    domain::UserUpdateCommand, repository::TransactionManager, use_case::UserUpdateUsecase,
};

use super::UserControllerError;

pub async fn update_user<TM, Usecase>(
    params: web::Path<UpdateUserPathParams>,
    info: web::Json<UpdateUserRequestJdto>,
    tx_manager: web::Data<Mutex<TM>>,
    usecase: web::Data<Usecase>,
) -> Result<web::Json<()>, actix_web::Error>
where
    Usecase: UserUpdateUsecase<TM>,
    TM: TransactionManager + Send,
{
    Ok(update_user_controller(
        tx_manager.as_ref(),
        usecase.as_ref(),
        params.into_inner(),
        info.into_inner(),
    )
    .await
    .map_err(|e| {
        println!("{e}");
        e
    })
    .map(web::Json)?)
}

async fn update_user_controller<Usecase, TM>(
    tx_manager: &Mutex<TM>,
    usecase: &Usecase,
    params: UpdateUserPathParams,
    info: UpdateUserRequestJdto,
) -> Result<(), UserControllerError>
where
    Usecase: UserUpdateUsecase<TM>,
    TM: TransactionManager + Send,
{
    let mut tx = TM::begin(tx_manager).await?;
    let res = usecase
        .update(
            &mut tx,
            params.id,
            UserUpdateCommand {
                name: info.name,
                mail_address: info.email,
            },
        )
        .await;
    TM::execute(tx, res).await
}

#[derive(Deserialize, Debug)]
pub struct UpdateUserRequestJdto {
    name: Option<String>,
    email: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateUserPathParams {
    pub id: Uuid,
}
