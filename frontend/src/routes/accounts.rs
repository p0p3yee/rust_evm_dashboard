use yew::prelude::*;

#[function_component(Accounts)]
pub fn accounts() -> Html {
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
                <table class="table is-striped is-narrow is-hoverable is-fullwidth">
                    <thead>
                        <tr>
                            <th>{"Name"}</th>
                            <th>{"Address"}</th>
                            <th>{"Balance"}</th>
                            <th>{"Private Key"}</th>
                        </tr>
                    </thead>
                </table>
            </section>
        </div>
    }
}