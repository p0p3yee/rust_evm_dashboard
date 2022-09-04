use super::{request_get, request_post};
use crate::error::Error;
use crate::types::*;

pub async fn all() -> Result<ReqResponse<Vec<Account>>, Error> {
    request_get::<Vec<Account>>("account".to_string()).await
}

// Return new account address
pub async fn create(ep: Account) -> Result<ReqResponse<String>, Error> {
    request_post("account/new".to_string(), ep).await
}

// Return the new account name
pub async fn update(body: Account) -> Result<ReqResponse<Account>, Error> {
    request_post("account/update".to_string(), body).await
}
