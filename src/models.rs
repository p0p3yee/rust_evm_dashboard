use crate::schema::{accounts, endpoints};
use serde::{Deserialize, Serialize};

#[derive(Insertable)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub address: &'a str,
    pub name: &'a str,
    pub private_key: &'a str
}

#[derive(Debug, Serialize, Deserialize, Queryable, AsChangeset, PartialEq)]
pub struct Account {
    pub address: String,
    pub name: Option<String>,
    pub private_key: Option<String>
}

#[derive(Insertable)]
#[table_name = "endpoints"]
pub struct NewEndpoint<'a> {
    pub name: &'a str,
    pub url: &'a str,
    pub symbol: &'a str
}

#[derive(Debug, Serialize, Deserialize, Queryable, AsChangeset, PartialEq)]
pub struct Endpoint {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub symbol: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct NewEndpointReq {
    pub name: String,
    pub url: String,
    pub symbol: String
}