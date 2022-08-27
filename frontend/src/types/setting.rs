use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Endpoint {
    pub name: String,
    pub url: String,
    pub symbol: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Account {
    pub name: String,
    pub address: String,
    pub private_key: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct UserSetting {
    pub selected_endpoint: Endpoint,
}