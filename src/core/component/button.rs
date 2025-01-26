use serde::{Serialize, Deserialize};
use yew::prelude::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ButtonComponent {
    pub text: String,
    pub font_size: f64,
    pub color: String,
    pub background_color: String,
}

impl ButtonComponent {
    pub fn render_properties(&self) -> Html {
        html! {
            <div class="property-group">
                <h4>{"Button"}</h4>
                <div class="property-row">
                    <label>{"Text"}</label>
                    <input type="text" value={self.text.clone()} />
                </div>
                <div class="property-row">
                    <label>{"Font Size"}</label>
                    <input type="number" value={self.font_size.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Text Color"}</label>
                    <input type="text" value={self.color.clone()} />
                </div>
                <div class="property-row">
                    <label>{"Background Color"}</label>
                    <input type="text" value={self.background_color.clone()} />
                </div>
            </div>
        }
    }
} 