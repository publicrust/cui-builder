use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div class="container">
            <h1 class="title">{"Привет, Yew!"}</h1>
            <p>{"Это простое веб-приложение на Rust с использованием Yew."}</p>
            <p>{"Значение счетчика: "}{*counter}</p>
            <button class="button" {onclick}>{"Увеличить"}</button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
