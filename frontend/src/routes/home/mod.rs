use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
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
                                {"XXX Chain Status"}
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
                            <p class="title">{"#"}{12345}</p>
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