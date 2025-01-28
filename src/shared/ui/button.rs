use yew::prelude::*;
use crate::shared::lib::types::Color;

#[derive(Properties, Clone)]
pub struct ButtonProps {
    pub text: String,
    pub color: Option<Color>,
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
}

impl PartialEq for ButtonProps {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && 
        match (&self.color, &other.color) {
            (Some(c1), Some(c2)) => c1.r == c2.r && c1.g == c2.g && c1.b == c2.b && c1.a == c2.a,
            (None, None) => true,
            _ => false,
        } &&
        self.class == other.class
    }
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let ButtonProps {
        text,
        color,
        onclick,
        class,
    } = props.clone();

    let style = color.map(|c| format!("background-color: {}", c.to_hex_string()));

    html! {
        <button
            class={classes!("button", class)}
            style={style}
            onclick={onclick}
        >
            {text}
        </button>
    }
} 