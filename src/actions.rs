use crate::models::{NewAccount, Account, Endpoint, NewEndpoint};
use diesel::{prelude::*, result::Error};

// Return account address if success
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

// Return endpoint name if success
pub async fn create_endpoint<'a>(conn: &SqliteConnection, name: &'a str, url: &'a str, symbol: &'a str) -> Result<String, Error> {
    use crate::schema::endpoints;

    let new_endpoint = NewEndpoint {
        name: name,
        url: url,
        symbol: symbol
    };

    let result = diesel::insert_into(endpoints::table)
        .values(&new_endpoint)
        .execute(conn);
    println!("Create Result: {:?}", result);
    match result {
        Ok(_) => Ok(name.to_string()),
        Err(e) => Err(e),
    }
}

// Return new account name if success
pub async fn update_account_name(conn: &SqliteConnection, target_addr: &str, new_name: &str) -> Result<String, Error> {
    use crate::schema::accounts::dsl::*;

    let result = diesel::update(accounts.find(target_addr))
        .set(name.eq(new_name))
        // .filter(address.eq(target_addr))
        .execute(conn);

    // println!("Update Result: {:?}", result);
    
    match result {
        Ok(num) => {
            if num == 0 {
                return Err(Error::NotFound)
            }
            Ok(new_name.to_string())
        },
        Err(e) => Err(e)
    }
}

// Return the new endpoint if success
pub async fn update_endpoint_data(conn: &SqliteConnection, target_id: i32, new_name: &str, new_url: &str, new_symbol: &str) -> Result<Endpoint, Error> {
    use crate::schema::endpoints::dsl::*;

    let result = diesel::update(endpoints.find(target_id))
        .set((
            name.eq(new_name.to_string()),
            url.eq(new_url.to_string()),
            symbol.eq(new_symbol.to_string()),
        ))
        .execute(conn);
    
        match result {
            Ok(num) => {
                if num == 0 {
                    return Err(Error::NotFound)
                }
                Ok(Endpoint {
                    id: target_id,
                    name: new_name.to_string(),
                    url: new_url.to_string(),
                    symbol: new_symbol.to_string()
                })
            },
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