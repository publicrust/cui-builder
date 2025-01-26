mod components;

use components::infinite_canvas::InfiniteCanvas;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="app">
            <InfiniteCanvas />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
