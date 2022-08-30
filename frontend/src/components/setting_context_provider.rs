use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::services::{get_selected, set_selected, endpoints};
use crate::types::Endpoint;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(SettingContextProvider)]
pub fn setting_context_provider(props: &Props) -> Html {

    let selected_endpoint_ctx = use_state(Endpoint::default);
    let all_endpoints = use_async(async move {endpoints::all().await});
    
    {
        let all_endpoints = all_endpoints.clone();
        use_mount(move || {
            all_endpoints.run();
        })
    }

    {
        let selected_endpoint_ctx = selected_endpoint_ctx.clone();
        use_effect_with_deps(
            move |current_setting| {
                if let Some(settings) = &current_setting.data {
                    if settings.status == "error" {
                        set_selected(None);
                    } else {
                        if let Some(selected_index) = get_selected() {
                            if selected_index as usize >= settings.data.len() || selected_index < 0{
                                set_selected(None);
                            } else {
                                set_selected(Some(selected_index));
                                selected_endpoint_ctx.set(settings.data[selected_index as usize].clone());
                            }
                        }
                    }
                }
                ||()
            }, all_endpoints,
        )
    }

    html! {
        <ContextProvider<UseStateHandle<Endpoint>> context={selected_endpoint_ctx}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<Endpoint>>>
    }
}