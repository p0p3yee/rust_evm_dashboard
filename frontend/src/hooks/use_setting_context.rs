use yew::prelude::*;

use crate::types::Endpoint;

pub fn use_setting_context() -> UseStateHandle<Endpoint> {
    use_context::<UseStateHandle<Endpoint>>().unwrap()
}