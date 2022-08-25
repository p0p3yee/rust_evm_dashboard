use crate::models::{NewAccount, Account};
use crate::db::connect;
use diesel::prelude::*;

pub fn create_account(acc: &NewAccount) {
    use crate::schema::accounts::dsl::*;

    let connection = connect();

    diesel::insert_into(accounts)
        .values(acc)
        .execute(&connetion).
        expect("Error creating account");
}

// pub fn update_account(acc: &Account) {
//     use crate::schema::accounts::dsl::*;

    
// }