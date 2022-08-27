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
                    {"Quisque facilisis lectus est, a vehicula dui mollis at. Vivamus eget ligula eros. Mauris ultricies felis quis mauris pharetra tristique. Curabitur ut eros consectetur mauris imperdiet ullamcorper eu id libero. Suspendisse potenti. Maecenas egestas non nibh eu varius. Donec non cursus neque. Aenean euismod nibh nibh, at lobortis neque porta quis. Mauris nec pulvinar tellus, ut sodales enim."}
                </p>
                </div>
            </section>
            <section class="section is-large">
                <h1 class="title has-text-centered">{"Home Page"}</h1>
                
            </section>
        </div>
        
    }
}