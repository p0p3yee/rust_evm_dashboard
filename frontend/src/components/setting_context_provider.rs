use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::types::UserSetting;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(SettingContextProvider)]
pub fn setting_context_provider(props: &Props) -> Html {

    let setting_ctx = use_state(UserSetting::default);
    // let current_setting = use_async(async move {current().await});

    html! {
        <ContextProvider<UseStateHandle<UserSetting>> context={setting_ctx}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<UserSetting>>>
    }
}