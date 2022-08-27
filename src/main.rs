#[macro_use]
extern crate diesel;

mod routes;
mod schema;
mod models;
mod actions;
mod apierror;

use actix_web::{web, App, HttpServer, middleware::Logger};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let backend_url = env::var("BACKEND_URL").expect("BACKEND_URL not found in .env");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found in .env");

    let pool_conn = Pool::builder()
        .build(ConnectionManager::<SqliteConnection>::new(&db_url))
        .expect(&format!("Error connecting to DB: {}", db_url));

    println!("Starting Backend on {}", backend_url);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())    
            .app_data(web::Data::new(pool_conn.clone()))
            .service(routes::get_endpoints)
            .service(routes::new_endpoint)
            .service(routes::update_endpoint)
            .service(routes::get_accounts)
            .service(routes::new_account)
            .service(routes::update_account)
            .service(routes::get_setting)
            .service(routes::init_setting)
            .service(routes::update_setting)
    })
    .bind(backend_url)?
    .run()
    .await
}