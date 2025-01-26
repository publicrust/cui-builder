use serde::{Serialize, Deserialize};
use yew::prelude::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TextComponent {
    pub text: String,
    pub font_size: f64,
    pub color: String,
    pub align: String,
}

impl TextComponent {
    pub fn render_properties(&self) -> Html {
        html! {
            <div class="property-group">
                <h4>{"Text"}</h4>
                <div class="property-row">
                    <label>{"Text"}</label>
                    <input type="text" value={self.text.clone()} />
                </div>
                <div class="property-row">
                    <label>{"Font Size"}</label>
                    <input type="number" value={self.font_size.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Color"}</label>
                    <input type="text" value={self.color.clone()} />
                </div>
                <div class="property-row">
                    <label>{"Align"}</label>
                    <input type="text" value={self.align.clone()} />
                </div>
            </div>
        }
    }
} 