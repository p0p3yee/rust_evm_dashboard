mod no_endpoint;
mod endpoint_list;
mod endpoint_form;

use endpoint_list::EndpointList;
use endpoint_form::EndpointForm;
use yew::prelude::*;
use yew_hooks::prelude::*;
use crate::{services::endpoints::all, types::Endpoint};

#[function_component(Endpoints)]
pub fn main_endpoints_view() -> Html {
    let endpoint_list = use_state(Vec::<Endpoint>::default);
    let get_endpoint_list = use_async_with_options(
        async move { all().await },
        UseAsyncOptions::enable_auto(),
    );

    {
        let endpoint_list = endpoint_list.clone();
        use_effect_with_deps(move |get_endpoint_list| {
            if get_endpoint_list.data.is_some() {
                let result = get_endpoint_list.data.as_ref().unwrap();
                if result.status == "success" {
                    endpoint_list.set(result.data.clone());
                }
            }
            || ()
        },
        get_endpoint_list.clone());
   }
   

    html!{
        <div>
            <section class="hero">
                <div class="hero-body">
                    <nav class="level">
                        <div class="level-item has-text-centered">
                            <div>
                                <p class="heading">{"Total Endpoints"}</p>
                                <p class="title">{ endpoint_list.len() }</p>
                            </div>
                        </div>
                    </nav>
                </div>
            </section>
            <div class="container">
                <section class="section">
                    <div class="tile is-ancestor">
                        <div class="tile is-parent is-vertical">
                            <article class="tile is-child notification">
                                <div class="content">
                                    <EndpointList epl={(*endpoint_list).clone()} />
                                </div>
                            </article>
                        </div>
                        <div class="tile is-parent is-vertical">
                            <div class="tile is-child notification">
                                <article class="tile is-child notification">
                                    <div class="content">
                                        <EndpointForm />
                                    </div>
                                </article>
                            </div>
                        </div>
                    </div>
                </section>
            </div>   
        </div>                 
    }
}