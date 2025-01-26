use serde::{Serialize, Deserialize};
use yew::prelude::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ImageComponent {
    pub sprite: Option<String>,
    pub color: Option<String>,
    pub material: Option<String>,
}

impl ImageComponent {
    pub fn render_properties(&self) -> Html {
        html! {
            <div class="property-group">
                <h4>{"Image"}</h4>
                <div class="property-row">
                    <label>{"Sprite"}</label>
                    <input type="text" value={self.sprite.clone().unwrap_or_default()} />
                </div>
                <div class="property-row">
                    <label>{"Color"}</label>
                    <input type="text" value={self.color.clone().unwrap_or_default()} />
                </div>
                <div class="property-row">
                    <label>{"Material"}</label>
                    <input type="text" value={self.material.clone().unwrap_or_default()} />
                </div>
            </div>
        }
    }
} 