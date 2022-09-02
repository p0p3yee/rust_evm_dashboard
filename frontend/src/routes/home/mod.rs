use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::hooks::use_setting_context;
use crate::services::web3::Web3Service;

#[function_component(Home)]
pub fn home() -> Html {

    let setting_ctx = use_setting_context();
    let w3 = use_state(Web3Service::default);
    let block_height = use_state(u64::default);

    {
        let setting_ctx = setting_ctx.clone();
        let w3 = w3.clone();
        let block_height = block_height.clone();
        
        use_effect_with_deps(
            move |setting_ctx| {
                if !setting_ctx.url.clone().is_empty() {
                    let set_ctx = setting_ctx.clone();
                    let block_height = block_height.clone();
                    spawn_local(async move {
                        let w3s = Web3Service::new(
                            set_ctx.url.clone(),
                            set_ctx.chain_id.clone()
                        ).await;
                        if let Ok(okw3s) = w3s {
                            let okw3sc = okw3s.clone();
                            w3.set(okw3s);
                            if let Ok(result) = okw3sc.block_height().await {
                                block_height.set(result);
                            }
                        }
                    });
                }
                || ()
            },
            setting_ctx
        )
    }
    
    html! {
        <div>
            <section class="hero is-info">
                <div class="hero-body has-text-centered">
                <p class="title">
                    {"Rust EVM Dashboard"}
                </p>
                <p class="subtitle">
                    {"Manage ALL your accounts on ALL EVM chain on this platform."}
                </p>
                </div>
            </section>

            <div class="container">
                <section class="section">
                    <nav class="level">
                        <div class="level-item has-text-centered">
                            <h1 class="title">
                                {setting_ctx.name.clone()}
                                {" Chain Status"}
                            </h1>
                        </div>
                    </nav>
                </section>
            </div>

            <section class="section">
                <nav class="level">
                    <div class="level-item has-text-centered">
                        <div>
                            <p class="heading">{"Current Block"}</p>
                            <p class="title">{"#"}{block_height.clone().to_string()}</p>
                        </div>
                        </div>
                        <div class="level-item has-text-centered">
                        <div>
                            <p class="heading">{"ABC"}</p>
                            <p class="title">{123}</p>
                        </div>
                        </div>
                        <div class="level-item has-text-centered">
                        <div>
                            <p class="heading">{"Def"}</p>
                            <p class="title">{"555"}</p>
                        </div>
                        </div>
                        <div class="level-item has-text-centered">
                        <div>
                            <p class="heading">{"Ghi"}</p>
                            <p class="title">{"abcde"}</p>
                        </div>
                    </div>
                </nav>   
            </section>

            <div class="container">
                <section class="section is-medium">
                </section>
            </div>
            
        </div>
        
    }
}