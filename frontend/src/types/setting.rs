use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct UserSetting {
    pub selected_endpoint_id: i32,
}