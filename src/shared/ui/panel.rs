use yew::prelude::*;
use crate::shared::lib::types::Color;

#[derive(Properties, Clone)]
pub struct PanelProps {
    pub children: Children,
    pub background_color: Option<Color>,
    #[prop_or_default]
    pub class: Classes,
}

impl PartialEq for PanelProps {
    fn eq(&self, other: &Self) -> bool {
        self.children == other.children &&
        match (&self.background_color, &other.background_color) {
            (Some(c1), Some(c2)) => c1.r == c2.r && c1.g == c2.g && c1.b == c2.b && c1.a == c2.a,
            (None, None) => true,
            _ => false,
        } &&
        self.class == other.class
    }
}

#[function_component(Panel)]
pub fn panel(props: &PanelProps) -> Html {
    let PanelProps {
        children,
        background_color,
        class,
    } = props.clone();

    let style = background_color.map(|c| format!("background-color: {}", c.to_hex_string()));

    html! {
        <div class={classes!("panel", class)} style={style}>
            {children}
        </div>
    }
} 