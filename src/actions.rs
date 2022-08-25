use crate::models::{NewAccount, Account};
use diesel::prelude::*;

pub fn create_account<'a>(conn: &SqliteConnection, addr: &'a str, name: &'a str, pkey: &'a str) {
    use crate::schema::accounts;

    let new_acc = NewAccount {
        address: addr,
        name: name,
        private_key: pkey,
    };

    diesel::insert_into(accounts::table)
        .values(&new_acc)
        .execute(conn)
        .expect("Error creating account");
}

// pub fn update_account_name(conn: &SqliteConnection, target_addr: &'a str, new_name: &'a str) {
//     use crate::schema::accounts::dsl::*;

//     let acc = diesel::update(accounts.find(target_addr))
//         .set(name.eq(new_name))
//         .get_result::<Account>(conn)
//         .expect(&format!("Target address not found"));
// } 

pub fn get_all_accounts(conn: &SqliteConnection) -> Result<Vec<Account>, diesel::result::Error> {
    use crate::schema::accounts::dsl::accounts;
    
    let result = accounts
        .load::<Account>(conn)?;

    Ok(result)
}