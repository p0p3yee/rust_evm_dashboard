use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Endpoint {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub symbol: String,
    pub chain_id: String,
    pub explorer_url: Option<String>,
}

impl Endpoint {
    pub fn is_none(&self) -> bool {
        self.name.is_empty() && self.url.is_empty() && self.symbol.is_empty()
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct UpdateEndpointReq {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub symbol: String,
    pub chain_id: String,
    pub explorer_url: Option<String>,
}