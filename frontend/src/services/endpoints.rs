use super::{request_get, request_post};
use crate::error::Error;
use crate::types::*;

pub async fn all() -> Result<ReqResponse<Vec<Endpoint>>, Error> {
    request_get::<Vec<Endpoint>>("endpoint".to_string()).await
}

// Return new endpoint name
pub async fn create(ep: Endpoint) -> Result<ReqResponse<String>, Error> {
    request_post("endpoint/new".to_string(), ep).await
}

// Return the new endpoint struct
pub async fn update(body: UpdateEndpointReq) -> Result<ReqResponse<Endpoint>, Error> {
    request_post("endpoint/update".to_string(), body).await
}
