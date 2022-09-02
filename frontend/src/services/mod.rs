pub mod setting;
pub mod requests;
pub mod endpoints;
pub mod web3;

pub use requests::{
    request_get, request_post, set_selected, get_selected
};