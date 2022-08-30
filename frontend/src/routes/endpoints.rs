use yew::prelude::*;

#[function_component(Endpoints)]
pub fn endpoints() -> Html {
    html! {
        <div>
            <section class="hero">
                <div class="hero-body">
                    <nav class="level">
                        <div class="level-item has-text-centered">
                            <div>
                                <p class="heading">{"Total Endpoints"}</p>
                                <p class="title">{0}</p>
                            </div>
                        </div>
                    </nav>
                </div>
            </section>
            <div class="container">
                <section class="section">
                    <div class="tile is-ancestor">
                        <div class="tile is-parent is-vertical">
                            <article class="tile is-child notification has-background-white">
                                <div class="content">
                                    <article class="panel">
                                        <div class="panel-block">
                                            <p class="control has-icons-left">
                                                <input class="input" type="text" placeholder="Search" />
                                                <span class="icon is-left">
                                                    <i class="fas fa-search" aria-hidden="true"></i>
                                                </span>
                                            </p>
                                        </div>
                                        <div class="panel-block">
                                            {"Ethereum Mainnet"}
                                        </div>
                                    </article>
                                </div>
                            </article>
                        </div>
                        <div class="tile is-parent is-vertical">
                            <div class="tile is-child notification">
                                <article class="tile is-child notification">
                                    <div class="content">
                                        <div class="field">
                                            <label class="label"> {"Name"} </label>
                                            <div class="control">
                                                <input class="input" type="text" placeholder="Endpoint Name" />
                                            </div>
                                        </div>
                
                                        <div class="field">
                                            <label class="label"> {"URL"} </label>
                                            <div class="control">
                                                <input class="input" type="text" placeholder="Endpoint URL" />
                                            </div>
                                        </div>
                    
                                        <div class="field">
                                            <label class="label"> {"Symbol"} </label>
                                            <div class="control">
                                                <input class="input" type="text" placeholder="Endpoint Symbol" />
                                            </div>
                                        </div>
                    
                                        <div class="field is-grouped">
                                            <div class="control">
                                                <button class="button is-danger"> {"Delete"} </button>
                                            </div>
                                            <div class="control">
                                                <button class="button is-link"> {"Save"} </button>
                                            </div>
                                            <div class="control">
                                                <button class="button is-link is-outlined"> {"Clear"} </button>
                                            </div> 
                                        </div>
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