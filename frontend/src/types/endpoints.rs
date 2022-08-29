use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Endpoint {
    pub name: String,
    pub url: String,
    pub symbol: String
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct UpdateEndpointReq {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub symbol: String
}