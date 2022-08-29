pub mod components;
pub mod error;
pub mod hooks;
pub mod routes;
pub mod types;
pub mod services;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{
    footer::Footer, header::Header, setting_context_provider::SettingContextProvider
};

use crate::routes::{switch, Route};

#[function_component(App)]
fn app() -> Html {
    html! {
        <SettingContextProvider>
            <BrowserRouter>
                <Header />
                <Switch<Route> render={Switch::render(switch)} />
                <Footer />
            </BrowserRouter>
        </SettingContextProvider>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}