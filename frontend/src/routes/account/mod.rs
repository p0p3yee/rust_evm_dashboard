use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct AccountProps {
    pub address: String
}

#[function_component(Account)]
pub fn account(props: &AccountProps) -> Html {
    let addr = props.address.clone();
    html! {
        <div>
            <section class="hero is-link">
                <div class="hero-body">
                    <nav class="level">
                        <div class="level-item has-text-centered">
                            <div>
                                <p class="heading">{"Address"}</p>
                                <p class="title">{ addr }</p>
                            </div>
                        </div>
                    </nav>
                    <nav class="level">
                        <div class="level-item has-text-centered">
                            <div>
                                <p class="heading">{"Balance"}</p>
                                <p class="title">{0}{" "}{"ETH"}</p>
                            </div>
                        </div>

                        <div class="level-item has-text-centered">
                            <div>
                                <p class="heading">{"Balance in USD"}</p>
                                <p class="title">{0}{" "}{"$USD"}</p>
                            </div>
                        </div>
                    </nav>   
                </div>
            </section>
            <div class="container">
                
                <section class="section has-text-centered">
                    <nav class="level">
                        <div class="level-item has-text-centered">
                            <div>
                                <p class="heading">{"Token Owned"}</p>
                                <p class="title">{0}</p>
                            </div>
                        </div>

                        <div class="level-item has-text-centered">
                            <div>
                                <p class="heading">{"Token Balance in USD"}</p>
                                <p class="title">{0}{" "}{"$USD"}</p>
                            </div>
                        </div>
                    </nav>   
                </section>

                <section class="section has-text-centered">
                    <table class="table is-striped is-narrow is-hoverable is-fullwidth">
                        <thead>
                            <tr>
                                <th>{"#"}</th>
                                <th>{"Token Name (Symbol)"}</th>
                                <th>{"Balance"}</th>
                                <th>{"Estimated Value in $USD"}</th>
                            </tr>
                        </thead>
                    </table>
                </section>
            </div>
        </div> 
    }
}