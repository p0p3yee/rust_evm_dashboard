use crate::schema::{accounts, endpoints, settings};
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
    pub symbol: &'a str,
    pub chain_id: &'a str,
    pub explorer_url: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Queryable, AsChangeset, PartialEq)]
pub struct Endpoint {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub symbol: String,
    pub chain_id: String,
    pub explorer_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct NewEndpointReq {
    pub name: String,
    pub url: String,
    pub symbol: String,
    pub chain_id: String,
    pub explorer_url: Option<String>,
}

#[derive(Insertable)]
#[table_name = "settings"]
pub struct NewSetting {
    pub selected_endpoint_id: i32
}

#[derive(Debug, Serialize, Deserialize, Queryable, AsChangeset, PartialEq)]
pub struct Setting {
    pub selected_endpoint_id: i32
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UpdateSettingReq {
    pub from_endpoint_id: i32,
    pub to_endpoint_id: i32
}