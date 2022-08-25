#[macro_use]
extern crate diesel;

mod routes;
mod schema;
mod models;
mod actions;

use actix_web::{web, App, HttpServer};
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

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool_conn.clone()))
            .service(routes::get_endpoints)
            .service(routes::get_accounts)
    })
    .bind(backend_url)?
    .run()
    .await
}