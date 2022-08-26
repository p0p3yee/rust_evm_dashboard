use crate::models::{NewAccount, Account, Endpoint};
use diesel::{prelude::*, result::Error};

pub async fn create_account<'a>(conn: &SqliteConnection, addr: &'a str, name: &'a str, pkey: &'a str) -> Result<String, Error> {
    use crate::schema::accounts;

    let new_acc = NewAccount {
        address: addr,
        name: name,
        private_key: pkey,
    };

    let result = diesel::insert_into(accounts::table)
        .values(&new_acc)
        .execute(conn);
    
    match result {
        Ok(_) => Ok(addr.to_string()),
        Err(e) => Err(e),
    }
}

pub async fn update_account_name(conn: &SqliteConnection, target_addr: &str, new_name: &str) -> Result<String, Error> {
    use crate::schema::accounts::dsl::*;

    let result = diesel::update(accounts.find(target_addr))
        .set(name.eq(new_name))
        .filter(address.eq(target_addr))
        .execute(conn);
    
    match result {
        Ok(_) => Ok(new_name.to_string()),
        Err(e) => Err(e)
    }
} 

pub async fn get_all_accounts(conn: &SqliteConnection) -> Result<Vec<Account>, diesel::result::Error> {
    use crate::schema::accounts::dsl::accounts;
    accounts.load::<Account>(conn)
}

pub async fn get_all_endpoints(conn: &SqliteConnection) -> Result<Vec<Endpoint>, diesel::result::Error> {
    use crate::schema::endpoints::dsl::endpoints;
    endpoints.load::<Endpoint>(conn)
}