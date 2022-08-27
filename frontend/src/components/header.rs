use yew::{function_component, html};
use yew_router::prelude::*;

use crate::routes::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <nav class="navbar" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <a class="navbar-item">
                    <Link<Route> to={Route::Home}>{ "Rust EVM Dashboard" }</Link<Route>>
                </a>
                <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </a>
            </div>
            
            <div class="navbar-menu">
                <div class="navbar-start">
                    <a class="navbar-item">
                        { "Home" }
                    </a>

                    <a class="navbar-item">
                        { "Item1" }
                    </a>

                    <a class="navbar-item">
                        { "Item 2" }
                    </a>
                </div>
            </div>

            <div class="navbar-end">
                <div class="navbar-item">
                    { "Connected to: " } {"Name"}
                </div>
                <div class="navbar-item">
                    <a class="button is-warning">
                        { "Switch" }
                    </a>
                </div>
            </div>
        </nav>
    }
}