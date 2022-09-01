use yew::{function_component, html};
use yew_router::prelude::*;

use crate::hooks::use_setting_context;
use crate::routes::Route;

#[function_component(Header)]
pub fn header() -> Html {
    let setting_ctx = use_setting_context();
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
                        <Link<Route> to={Route::Home}>
                            { "Home" }
                        </Link<Route>>
                    </a>

                    <a class="navbar-item">
                        <Link<Route> to={Route::Accounts}>
                            { "Accounts" }
                        </Link<Route>>
                    </a>

                    <a class="navbar-item">
                        <Link<Route> to={Route::Endpoints}>
                            { "Endpoints" }
                        </Link<Route>>
                    </a>
                </div>
            </div>

            <div class="navbar-end">
                <div class="navbar-item">
                    {
                        if !setting_ctx.is_none() {
                            { "Connected to:"  }
                        } else {
                            { "Not available" }
                        }
                    }
                    <strong>
                        {
                            if !setting_ctx.is_none() {
                                { setting_ctx.name.clone() }
                            } else {
                                { "".to_string() }
                            }
                        }
                    </strong>
                </div>
                <div class="navbar-item">
                    <Link<Route> to={Route::Endpoints}>
                        <a class={
                            if setting_ctx.is_none() {
                                "button is-success"
                            } else {
                                "button is-warning"
                            }
                        }>
                        {
                            if setting_ctx.is_none() {
                                { "Add Endpoint" }
                            } else {
                                { "Switch" }   
                            }
                        }
                        </a>
                    </Link<Route>>
                </div>
            </div>
        </nav>
    }
}