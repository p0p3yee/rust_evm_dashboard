use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::types::{Account, form::FormFieldState};
use crate::services::accounts::create;
use crate::services::web3::{is_valid_address, is_valid_secret_key};

#[derive(Clone, Default)]
struct FormState {
    name: FormFieldState,
    addr: FormFieldState,
    pkey: FormFieldState,
}

impl FormState {
    fn is_all_valid(&self) -> bool {
        (self.name.is_valid() || self.name.is_default()) && self.addr.is_valid() && (self.pkey.is_default() || self.pkey.is_valid())
    }
}

#[function_component(AddAccount)]
pub fn add_account() -> Html {

    let form_state = use_state(FormState::default);

    let acc_info = use_state(Account::default);
    let create_acc = {
        let acc_info = acc_info.clone();
        use_async( async move {
            create((*acc_info).clone()).await
        })
    };

    {
        use_effect_with_deps(
            move |create_acc| {
                if let Some(ac_info) = &create_acc.data {
                    log::info!("Added Account, resp: {:?}", ac_info);
                    if ac_info.status == "error" {
                        log::info!("Error on creating acc, reason: {:?}", ac_info.data);
                    } else {
                        log::info!("Successfully added new acc, addr: {:?}", ac_info.data);
                    }
                }
                || ()
            },
            create_acc.clone()
        );
    }

    let onsubmit = {
        let create_acc = create_acc.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            //TODO: Add encrypting private key if not empty
            create_acc.run()
        })
    };

    let on_input_name = {
        let acc_info = acc_info.clone();
        let form_state = form_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*acc_info).clone();
            let input_val = input.value();

            let mut new_form_state = (*form_state).clone();

            if !input_val.is_empty() {
                new_form_state.name = FormFieldState::ValidState;
            } else {
                new_form_state.name = FormFieldState::DefaultState;
            }

            info.name = Some(input_val);
            acc_info.set(info);

            form_state.set(new_form_state);
        })
    };

    let on_input_addr = {
        let acc_info = acc_info.clone();
        let form_state = form_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*acc_info).clone();
            let input_val = input.value();

            let mut new_form_state = (*form_state).clone();

            if input_val.is_empty() {
                new_form_state.addr = FormFieldState::ErrorState("Address can't be empty".to_string());
            } else {
                // Check if address is valid
                if is_valid_address(input_val.clone()) {
                    new_form_state.addr = FormFieldState::ValidState;
                } else {
                    new_form_state.addr = FormFieldState::ErrorState("Invalid address".to_string());
                }
            }

            info.address = input_val;
            acc_info.set(info);

            form_state.set(new_form_state);
        })
    };

    let on_input_pkey = {
        let acc_info = acc_info.clone();
        let form_state = form_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*acc_info).clone();
            let input_val = input.value();

            let mut new_form_state = (*form_state).clone();

            if !input_val.is_empty() {
                // Check if pkey is valid
                if is_valid_secret_key(input_val.clone()) {
                    new_form_state.pkey = FormFieldState::ValidState;
                } else {
                    new_form_state.pkey = FormFieldState::ErrorState("Invalid private key".to_string());
                }
            } else {
                new_form_state.pkey = FormFieldState::DefaultState;
            }

            info.private_key = Some(input_val);
            
            acc_info.set(info);

            form_state.set(new_form_state);
        })
    };


    html! {
        <form {onsubmit}>
            <div class="columns">
                <div class="column is-2">
                    <div class="field">
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
                                placeholder="Name"
                                value={acc_info.name.clone()}
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
                </div>

                <div class="column is-4">
                    <div class="field">
                        <div class="control">
                            <input
                                class={
                                    if form_state.addr.is_error() { 
                                        "input is-danger"
                                    } else if form_state.addr.is_valid() {
                                        "input is-success"
                                    } else {
                                        "input"
                                    }
                                }
                                type="text"
                                placeholder="Address"
                                value={acc_info.address.clone()}
                                oninput={on_input_addr} 
                            />
                        </div>
                        {
                            if form_state.addr.is_error() {
                                html! {
                                    <p class="help is-danger">{ form_state.addr.get_error_msg() }</p>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                </div>

                <div class="column is-4">
                    <div class="field">
                        <div class="control">
                            <input
                                class={
                                    if form_state.pkey.is_error() { 
                                        "input is-danger"
                                    } else if form_state.pkey.is_valid() {
                                        "input is-success"
                                    } else {
                                        "input"
                                    }
                                }
                                type="password"
                                placeholder="Private Key (Excluding 0x prefix)"
                                value={acc_info.private_key.clone()}
                                oninput={on_input_pkey} 
                            />
                        </div>
                        {
                            if form_state.pkey.is_error() {
                                html! {
                                    <p class="help is-danger">{ form_state.pkey.get_error_msg() }</p>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                </div>

                <div class="column">
                    <div class="field is-grouped">
                        <div class="control">
                            <button 
                                class="button is-outlined is-success"
                                type="submit"
                                disabled={!form_state.is_all_valid()}
                            >
                                {"Add"}
                            </button>
                        </div>

                        <div class="control">
                            <button
                                class="button is-outlined is-danger"
                                type="reset"
                            >
                                {"Cancel"}
                            </button>
                        </div>
                    </div>
                </div>
            </div>
            
        </form>
    }
}