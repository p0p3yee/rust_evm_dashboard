use yew::prelude::*;
use yew_hooks::prelude::*;
use web_sys::HtmlInputElement;
use reqwest::Url;

use crate::types::{Endpoint, form::FormFieldState};
use crate::services::endpoints::create;

#[derive(Clone, Default)]
struct FormState {
    name: FormFieldState,
    symbol: FormFieldState,
    chain_id: FormFieldState,
    rpc_url: FormFieldState,
    explorer_url: FormFieldState,
}

impl FormState {
    fn is_all_valid(&self) -> bool {
        self.name.is_valid() && self.chain_id.is_valid() && self.rpc_url.is_valid() && (self.explorer_url.is_valid() || self.explorer_url.is_default())
    }
}

#[function_component(EndpointForm)]
pub fn endpoint_form() -> Html {

    let form_state = use_state(FormState::default);

    let endpoint_info = use_state(Endpoint::default);
    let create_endpoint = {
        let endpoint_info = endpoint_info.clone();
        use_async( async move {
            create((*endpoint_info).clone()).await
        })
    };

    {
        use_effect_with_deps(
            move |create_endpoint| {
                if let Some(ep_info) = &create_endpoint.data {
                    log::info!("Added endpoint resp: {:?}", ep_info);
                    if ep_info.status == "error" {
                        log::info!("Error on creating endpoint, reason: {:?}", ep_info.data);
                    } else {
                        log::info!("Successfully Added new endpoint, name: {:?}", ep_info.data);
                    }
                }
                || ()
            },
            create_endpoint.clone(),
        );
    }

    let onsubmit = {
        let create_endpoint = create_endpoint.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            // TODO: Get block height using the url to see if it is valid
            create_endpoint.run()
        })
    };

    let on_input_name = {
        let endpoint_info = endpoint_info.clone();
        let form_state = form_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*endpoint_info).clone();
            let input_val = input.value();

            let mut new_form_state = (*form_state).clone();
            
            if input_val.is_empty() {
                new_form_state.name = FormFieldState::ErrorState("Endpoint name can't be empty".to_string());
            } else {
                new_form_state.name = FormFieldState::ValidState;
            }

            info.name = input_val;
            endpoint_info.set(info);

            form_state.set(new_form_state);
        })
    };

    let on_input_url = {
        let endpoint_info = endpoint_info.clone();
        let form_state = form_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*endpoint_info).clone();
            let input_val = input.value();

            let mut new_form_state = (*form_state).clone();

            if input_val.is_empty() {
                new_form_state.rpc_url = FormFieldState::ErrorState("Endpoint RPC URL can't be empty".to_string());
            } else {
                if input_val.starts_with("ws") { // Not accepting websocket connection
                    new_form_state.rpc_url = FormFieldState::ErrorState("WS connection is not supported, use http instead.".to_string());
                } else {
                    if let Err(_) = Url::parse(&input_val) {
                        new_form_state.rpc_url = FormFieldState::ErrorState("Invalid URL, it should starts with http or https.".to_string());
                    } else {
                        new_form_state.rpc_url = FormFieldState::ValidState;
                    }
                }
            }

            info.url = input.value();
            endpoint_info.set(info);

            form_state.set(new_form_state);
        })
    };

    let on_input_symbol = {
        let endpoint_info = endpoint_info.clone();
        let form_state = form_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*endpoint_info).clone();
            let input_val = input.value();

            let mut new_form_state = (*form_state).clone();

            if input_val.is_empty() {
                new_form_state.symbol = FormFieldState::ErrorState("Endpoint Symbol can't be empty".to_string());
            } else {
                new_form_state.symbol = FormFieldState::ValidState;
            }

            info.symbol = input.value();
            endpoint_info.set(info);

            form_state.set(new_form_state);
        })
    };

    let on_input_explorer_url = {
        let endpoint_info = endpoint_info.clone();
        let form_state = form_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*endpoint_info).clone();
            let input_val = input.value();

            let mut new_form_state = (*form_state).clone();

            if !input_val.is_empty() {
                if let Err(_) = Url::parse(&input_val) {
                    new_form_state.explorer_url = FormFieldState::ErrorState("Invalid Explorer URL".to_string());
                } else {
                    new_form_state.explorer_url = FormFieldState::ValidState;
                }
            } else {
                new_form_state.explorer_url = FormFieldState::DefaultState;
            }

            info.explorer_url = Some(input.value());
            endpoint_info.set(info);

            form_state.set(new_form_state);
        })
    };

    let on_input_chain_id = {
        let endpoint_info = endpoint_info.clone();
        let form_state = form_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*endpoint_info).clone();
            let input_val = input.value();

            let mut new_form_state = (*form_state).clone();
            if input_val.is_empty() {
                new_form_state.chain_id = FormFieldState::ErrorState("Chain ID can not be empty".to_string());
            } else {
                let parse_result = input_val.parse::<i32>();
                if parse_result.is_err() {
                    new_form_state.chain_id = FormFieldState::ErrorState("Invalid Chain ID".to_string());
                } else {
                    new_form_state.chain_id = FormFieldState::ValidState;
                }
            }

            info.chain_id = input.value();
            endpoint_info.set(info);

            form_state.set(new_form_state);
        })
    };

    html! {
        <form {onsubmit}>
            <div class="field">
                <label class="label"> {"Name"} </label>
                <div class="control">
                    <input 
                        class={
                            if form_state.name.is_error() { 
                                "input is-danger"
                            } else if form_state.name.is_valid() {
                                "input is-success"
                            } else {
                                "input"
                            }
                        } 
                        type="text" 
                        placeholder="Endpoint Name"
                        value={endpoint_info.name.clone()}
                        oninput={on_input_name} 
                    />
                </div>
                {
                    if form_state.name.is_error() {
                        html! {
                            <p class="help is-danger">{ form_state.name.get_error_msg() }</p>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>

            <div class="field">
                <label class="label"> {"Chain ID"} </label>
                <div class="control">
                    <input 
                        class={
                            if form_state.chain_id.is_error() { 
                                "input is-danger"
                            } else if form_state.chain_id.is_valid() {
                                "input is-success"
                            } else {
                                "input"
                            }
                        }
                        type="text" 
                        placeholder="Endpoint Chain ID" 
                        value={endpoint_info.chain_id.clone()}
                        oninput={on_input_chain_id}
                    />
                    {
                        if form_state.chain_id.is_error() {
                            html! {
                                <p class="help is-danger">{ form_state.chain_id.get_error_msg() }</p>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
            </div>

            <div class="field">
                <label class="label"> {"URL"} </label>
                <div class="control">
                    <input 
                        class={
                            if form_state.rpc_url.is_error() { 
                                "input is-danger"
                            } else if form_state.rpc_url.is_valid() {
                                "input is-success"
                            } else {
                                "input"
                            }
                        }
                        type="text" 
                        placeholder="Endpoint URL"
                        value={endpoint_info.url.clone()}
                        oninput={on_input_url}
                    />
                    {
                        if form_state.rpc_url.is_error() {
                            html! {
                                <p class="help is-danger">{ form_state.rpc_url.get_error_msg() }</p>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
            </div>

            <div class="field">
                <label class="label"> {"Symbol"} </label>
                <div class="control">
                    <input 
                        class={
                            if form_state.symbol.is_error() { 
                                "input is-danger"
                            } else if form_state.symbol.is_valid() {
                                "input is-success"
                            } else {
                                "input"
                            }
                        }
                        type="text"
                        placeholder="Endpoint Symbol"
                        value={endpoint_info.symbol.clone()}
                        oninput={on_input_symbol} 
                    />
                    {
                        if form_state.symbol.is_error() {
                            html! {
                                <p class="help is-danger">{ form_state.symbol.get_error_msg() }</p>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
            </div>

            <div class="field">
                <label class="label"> {"Explorer URL"} </label>
                <div class="control">
                    <input 
                        class={
                            if form_state.explorer_url.is_error() { 
                                "input is-danger"
                            } else if form_state.explorer_url.is_valid() {
                                "input is-success"
                            } else {
                                "input"
                            }
                        }
                        type="text" 
                        placeholder="Explorer URL" 
                        value={endpoint_info.explorer_url.clone()}
                        oninput={on_input_explorer_url}
                    />
                    {
                        if form_state.explorer_url.is_error() {
                            html! {
                                <p class="help is-danger">{ form_state.explorer_url.get_error_msg() }</p>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
            </div>

            <div class="field is-grouped">
                <div class="control">
                    <button class="button is-link" type="submit" disabled={!form_state.is_all_valid()}> {"Save"} </button>
                </div>
                <div class="control">
                    <button class="button is-link is-outlined" type="reset"> {"Clear"} </button>
                </div> 
            </div>
        </form>
    }
}