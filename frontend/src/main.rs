use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    HomeView
}

#[function_component(HomeView)]
fn home_view() -> Html {
    return html! {
        <>
            {"Rust EVM Dashboard"}
        </>
    }
}

fn switch(route: &Route) -> Html {
    match route {
        Route::HomeView => html! {
            <HomeView/>
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}