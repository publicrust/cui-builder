use serde::{Serialize, Deserialize};
use yew::prelude::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RectTransformComponent {
    pub anchor_min: (f64, f64),
    pub anchor_max: (f64, f64),
    pub offset_min: (f64, f64),
    pub offset_max: (f64, f64),
}

impl RectTransformComponent {
    pub fn render_properties(&self) -> Html {
        html! {
            <div class="property-group">
                <h4>{"RectTransform"}</h4>
                <div class="property-row">
                    <label>{"Anchor Min"}</label>
                    <input type="number" value={self.anchor_min.0.to_string()} />
                    <input type="number" value={self.anchor_min.1.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Anchor Max"}</label>
                    <input type="number" value={self.anchor_max.0.to_string()} />
                    <input type="number" value={self.anchor_max.1.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Offset Min"}</label>
                    <input type="number" value={self.offset_min.0.to_string()} />
                    <input type="number" value={self.offset_min.1.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Offset Max"}</label>
                    <input type="number" value={self.offset_max.0.to_string()} />
                    <input type="number" value={self.offset_max.1.to_string()} />
                </div>
            </div>
        }
    }
} 