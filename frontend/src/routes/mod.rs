pub mod home;
pub mod endpoints;
pub mod accounts;
pub mod account;

use home::Home;
use endpoints::Endpoints;
use accounts::Accounts;
use account::Account;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/endpoints")]
    Endpoints,

    #[at("/accounts")]
    Accounts,

    #[at("/account/:address")]
    Account { address: String },

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {<Home />},
        Route::Endpoints => html! {<Endpoints />},
        Route::Accounts => html! {<Accounts />},
        Route::Account { address } => html! {
            <Account address={address.clone()} />
        },
        Route::NotFound => html! { "Page not found" },
    }
}