use crate::actions::*;
use crate::models::{ Account, Endpoint, NewEndpointReq, Setting, UpdateSettingReq };
use crate::Pool;
use actix_web::HttpResponse;
use serde_json::json;
use actix_web::{ http::header::ContentType, web, get, post };

fn response_builder<T: serde::Serialize>(is_error: bool, data: T) -> HttpResponse {
    let status = match is_error {
        true => "error",
        false => "success"
    };
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json!({
            "status": status,
            "data": data
        }).to_string())
}

#[get("/setting")]
pub async fn get_setting(pool: web::Data<Pool>) -> HttpResponse {
    let db_conn = pool.get().unwrap();
    match get_current_setting(&db_conn).await {
        Ok(s) => response_builder(false, s),
        Err(e) => response_builder(true, e.to_string())
    }
}

#[post("/setting/init")]
pub async fn init_setting(s: web::Json<Setting>, pool: web::Data<Pool>) -> HttpResponse {
    let db_conn = pool.get().unwrap();
    let setting = s.into_inner();
    println!("Init setting with endpoint id: {:?}", setting.selected_endpoint_id);
    match inititialize_setting(&db_conn, setting.selected_endpoint_id).await {
        Ok(id) => response_builder(false, id),
        Err(e) => response_builder(true, e.to_string())
    }
}

#[post("/setting/update")]
pub async fn update_setting(s: web::Json<UpdateSettingReq>, pool: web::Data<Pool>) -> HttpResponse {
    let db_conn = pool.get().unwrap();
    let setting = s.into_inner();
    println!("Update setting from endpoint id: {:?} to id: {:?}", setting.from_endpoint_id, setting.to_endpoint_id);
    match update_current_setting(&db_conn, setting.from_endpoint_id, setting.to_endpoint_id).await {
        Ok(id) => response_builder(false, id),
        Err(e) => response_builder(true, e.to_string())
    }
}

#[get("/endpoint")]
pub async fn get_endpoints(pool: web::Data<Pool>) -> HttpResponse {
    let db_conn = pool.get().unwrap();
    match get_all_endpoints(&db_conn).await {
        Ok(esp) => response_builder(false, esp),
        Err(e) => response_builder(true, e.to_string())
    }
}

#[post("/endpoint/new")]
pub async fn new_endpoint(ep: web::Json<NewEndpointReq>, pool: web::Data<Pool>) -> HttpResponse {
    let db_conn = pool.get().unwrap();
    let ep = ep.into_inner();
    println!("New Endpoint: {:?}", ep);
    match create_endpoint(
        &db_conn,
        &ep.name,
        &ep.url,
        &ep.symbol,
    ).await {
        Ok (name) => response_builder(false, name),
        Err (e) => {
            println!("Error in creating endpoint: {:?}", e);
            response_builder(true, e.to_string())
        }
    }
}

#[post("/endpoint/update")]
pub async fn update_endpoint(ep: web::Json<Endpoint>, pool: web::Data<Pool>) -> HttpResponse {
    let db_conn = pool.get().unwrap();
    let ep = ep.into_inner();

    match update_endpoint_data(
        &db_conn, 
        ep.id,
        &ep.name,
        &ep.url,
        &ep.symbol
    ).await {
        Ok (updated_ep) => response_builder(false, updated_ep),
        Err (e) => {
            println!("Error in updating endpoint: {:?}", e);
            response_builder(true, e.to_string())
        }
    }
}

#[get("/account")]
pub async fn get_accounts(pool: web::Data<Pool>) ->  HttpResponse {
    let db_conn = pool.get().unwrap();
    match get_all_accounts(&db_conn).await {
        Ok(accs) => response_builder(false, accs),
        Err(e) => response_builder(true, e.to_string()),
    }
}

#[post("/account/new")]
pub async fn new_account(acc: web::Json<Account>, pool: web::Data<Pool>) -> HttpResponse {
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
        Ok (addr) => response_builder(false, addr),
        Err (e) => {
            println!("Error in creating account: {:?}", e);
            response_builder(true, e.to_string())
        }
    }
}

#[post("/account/update")]
pub async fn update_account(acc: web::Json<Account>, pool: web::Data<Pool>) -> HttpResponse {
    let db_conn = pool.get().unwrap();
    let acc = acc.into_inner();
    let new_name = acc.name.unwrap_or_default();

    let result = update_account_name(
        &db_conn, 
        &acc.address,
        &new_name,
    ).await;

    match result {
        Ok (name) => response_builder(false, name),
        Err (e) => {
            println!("Error in updating account: {:?}", e);
            response_builder(true, e.to_string())
        }
    }
}