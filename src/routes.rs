// use crate::models::{Endpoint, Account};

use crate::actions::*;

use crate::Pool;
use actix_web::{ web, get, HttpResponse, Error};

#[get("/endpoint")]
pub async fn get_endpoints(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    
    Ok(HttpResponse::Ok().json(""))
}

#[get("/account")]
pub async fn get_accounts(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let db_conn = pool.get().unwrap();
    Ok(match get_all_accounts(&db_conn) {
        Ok(accs) => HttpResponse::Ok().json(accs),
        _ => HttpResponse::from(HttpResponse::InternalServerError())
    })
}