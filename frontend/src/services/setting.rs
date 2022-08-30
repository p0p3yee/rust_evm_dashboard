use super::{request_get, request_post};
use crate::error::Error;
use crate::types::*;

pub async fn current_setting() -> Result<ReqResponse<Vec<UserSetting>>, Error> {
    request_get::<Vec<UserSetting>>("setting".to_string()).await
}

// Return the selected endpoint id
pub async fn new(new_setting: UserSetting) -> Result<ReqResponse<i32>, Error> {
   request_post("setting/new".to_string(), new_setting).await
}