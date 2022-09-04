use yew::prelude::*;
use yew_router::prelude::Link;

use crate::types::{AccountDetail};
use crate::routes::Route;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub ac_list: Vec<AccountDetail>,
}

#[function_component(ListAccounts)]
pub fn list_accounts(props: &Props) -> Html {
    html! {
        <table class="table is-striped is-narrow is-hoverable is-fullwidth">
            <thead>
                <tr>
                    <th>{"Name"}</th>
                    <th>{"Address"}</th>
                    <th>{"Balance"}</th>
                    <th></th>
                    <th></th>
                </tr>
            </thead>
            <tbody>
                {
                    for props.ac_list.iter().map(|ac| {
                        html! {
                            <tr>
                                <th>
                                    {ac.name.to_string()}
                                </th>
                                <th>
                                    {ac.address.to_string()}
                                </th>
                                <th>
                                    {ac.balance.to_string()}
                                </th>
                                <th>
                                    <button 
                                        class="button is-outlined"
                                        disabled={ac.private_key.is_empty()}
                                    >
                                        {"Show Private Key"}
                                    </button>
                                </th>
                                <th>
                                    <button 
                                        class="button is-outlined"
                                    >
                                        <Link<Route> to={Route::Account {address: ac.address.to_string()} }>
                                            {"Detail"}
                                        </Link<Route>>
                                    </button>
                                </th>
                            </tr>
                        }
                    })
                }
            </tbody>
        </table>
    }
}