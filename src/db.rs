use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn connect() -> SqliteConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");

    SqliteConnection::establish(&db_url).expect(&format!("Error connecting to DB: {}", db_url))
}