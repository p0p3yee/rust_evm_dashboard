use yew::prelude::*;

#[function_component(NoEndpointList)]
pub fn no_endpoint_view() -> Html {
    html! {
        <div>
            <div class="subtitle has-text-centered">
                {"No Endpoint Found."}
            </div>
            <div class="subtitle has-text-centered">
                {"Add your first endpoint now"}
            </div>
        </div>
    }
}
