use super::{request_get};
use crate::error::Error;
use crate::types::*;

pub async fn current_setting() -> Result<ReqResponse<Vec<UserSetting>>, Error> {
    request_get::<Vec<UserSetting>>("setting".to_string()).await
}