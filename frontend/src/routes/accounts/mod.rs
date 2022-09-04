mod add_account;
mod list_accounts;

use add_account::AddAccount;
use list_accounts::ListAccounts;

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::types::AccountDetail;
use crate::services::accounts::all;
use crate::hooks::use_setting_context;

#[function_component(Accounts)]
pub fn accounts() -> Html {

    let setting_ctx = use_setting_context();
    
    let acc_list = use_state(Vec::<AccountDetail>::default);
    let get_acc_list = use_async_with_options(
        async move { all().await },
        UseAsyncOptions::enable_auto(),
    );

    {
        let acc_list = acc_list.clone();
        let setting_ctx = setting_ctx.clone();
        use_effect_with_deps(
            move |get_acc_list| {
                if let Some(result) = &get_acc_list.data {
                    if result.status == "success" {
                        let parsed_result: Vec<AccountDetail>  = result.data.iter().map(|each_ac| {
                            AccountDetail { 
                                name: each_ac.name.as_ref().unwrap_or(&"".to_string()).to_string(), 
                                address: each_ac.address.clone(), 
                                private_key: each_ac.private_key.as_ref().unwrap_or(&"".to_string()).to_string(), 
                                balance: format!("0 {}", setting_ctx.symbol).to_string(),
                            }
                        }).collect::<Vec<AccountDetail>>();
                        acc_list.set(parsed_result);
                    }
                }
                || ()
            }, 
            get_acc_list.clone()
        )
    }

    html! {
        <div class="container">
            <section class="section">
                <h1 class="title has-text-centered"> {"Account Summary"} </h1>
                <nav class="level">
                        <div class="level-item has-text-centered">
                            <div>
                                <p class="heading">{"Total Accounts"}</p>
                                <p class="title">{0}</p>
                            </div>
                        </div>
                        
                        <div class="level-item has-text-centered">
                            <div>
                                <p class="heading">{"Total Balance"}</p>
                                <p class="title">{0}{" "}{"ETH"}</p>
                            </div>
                        </div>

                        <div class="level-item has-text-centered">
                            <div>
                                <p class="heading">{"Total Value in USD"}</p>
                                <p class="title">{0}{" "}{"$USD"}</p>
                            </div>
                        </div>
                </nav>   
            </section>
            <section class="section">
                <AddAccount />
            </section>
            <section class="section">
                <ListAccounts ac_list={(*acc_list).clone()}/>
            </section>
        </div>
    }
}