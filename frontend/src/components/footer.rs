use yew::{function_component, html};
use yew_router::prelude::*;

use crate::routes::Route;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <div class="content has-text-centered">
                <Link<Route> to={Route::Home}>{ "Rust EVM Dashboard" }</Link<Route>>
                <span>
                    { "© 2022. Built by " }
                    <a href="https://github.com/p0p3yee"> { "p0p3yee" } </a>
                </span>
            </div>
        </footer>
    }
}