use crate::schema::{accounts, endpoints};

#[derive(Insertable)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub address: &'a str,
    pub name: &'a str,
    pub private_key: &'a str
}

#[derive(Debug, Queryable, AsChangeset)]
pub struct Account {
    pub address: String,
    pub name: String,
    pub private_key: String
}

#[derive(Insertable)]
#[table_name = "endpoints"]
pub struct NewEndpoint<'a> {
    pub name: &'a str,
    pub url: &'a str,
    pub symbol: &'a str
}

#[derive(Debug, Queryable, AsChangeset)]
pub struct Endpoint {
    pub name: String,
    pub url: String,
    pub symbol: String
}