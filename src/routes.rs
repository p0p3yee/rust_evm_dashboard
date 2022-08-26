use crate::actions::*;
use crate::models::{ Account, Endpoint };
use crate::Pool;
use crate::apierror::ApiError;
// use actix_web::{HttpRequest, Responder};
use actix_web::{ web::{ self, Json }, get, post };

#[get("/endpoint")]
pub async fn get_endpoints(pool: web::Data<Pool>) -> Result<Json<Vec<Endpoint>>, ApiError> {
    let db_conn = pool.get().unwrap();
    match get_all_endpoints(&db_conn).await {
        Ok(eps) => Ok(Json(eps)),
        _ => Err(ApiError::DatabaseInternalError)
    }
}

#[get("/account")]
pub async fn get_accounts(pool: web::Data<Pool>) ->  Result<Json<Vec<Account>>, ApiError> {
    let db_conn = pool.get().unwrap();
    match get_all_accounts(&db_conn).await {
        Ok(accs) => Ok(Json(accs)),
        _ => Err(ApiError::DatabaseInternalError),
    }
}

#[post("/account/new")]
pub async fn new_account(acc: web::Json<Account>, pool: web::Data<Pool>) -> Result<Json<String>, ApiError> {
    let db_conn = pool.get().unwrap();
    let acc = acc.into_inner();
    println!("New account: {:?}", acc);
    let result = create_account(
        &db_conn,
        &acc.address,
        &acc.name.unwrap_or_default(),
        &acc.private_key.unwrap_or_default(),
    ).await;

    match result {
        Ok (addr) => Ok(Json(addr.to_string())),
        Err (e) => {
            println!("Error in creating account: {:?}", e);
            Err(ApiError::AccountCreationFailure)
        }
    }
}

#[post("/account/update")]
pub async fn update_account(acc: web::Json<Account>, pool: web::Data<Pool>) -> Result<Json<String>, ApiError> {
    let db_conn = pool.get().unwrap();
    let acc = acc.into_inner();
    let new_name = acc.name.unwrap_or_default();

    let result = update_account_name(
        &db_conn, 
        &acc.address,
        &new_name,
    ).await;

    match result {
        Ok (name) => Ok(Json(name.to_string())),
        Err (e) => {
            println!("Error in updating account: {:?}", e);
            Err(ApiError::AccountNotFound)
        }
    }
}