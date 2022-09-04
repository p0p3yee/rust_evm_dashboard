use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Account {
    pub address: String,
    pub name: Option<String>,
    pub private_key: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct AccountDetail {
    pub name: String,
    pub address: String,
    pub private_key: String,
    pub balance: String,
}


impl Account {
    pub fn is_none(&self) -> bool {
        self.address.is_empty() && 
        (self.name.is_none() || (self.name.is_some() && self.name.clone().unwrap().is_empty())) &&
        (self.private_key.is_none() || (self.private_key.is_some() && self.private_key.clone().unwrap().is_empty()))
    }
}