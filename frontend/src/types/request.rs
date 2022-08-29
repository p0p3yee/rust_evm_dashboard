use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct ReqResponse <T>{
    pub status: String,
    pub data: T
}