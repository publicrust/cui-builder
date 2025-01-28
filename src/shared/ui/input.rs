use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct InputProps {
    pub value: String,
    pub onchange: Callback<String>,
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub class: Classes,
}

impl PartialEq for InputProps {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value &&
        self.placeholder == other.placeholder &&
        self.class == other.class
    }
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let InputProps {
        value,
        onchange,
        placeholder,
        class,
    } = props.clone();

    let oninput = {
        let onchange = onchange.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            onchange.emit(input.value());
        })
    };

    html! {
        <input
            type="text"
            {value}
            {oninput}
            placeholder={placeholder.unwrap_or_default()}
            class={classes!("input", class)}
        />
    }
} 