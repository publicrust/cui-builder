use serde::{Serialize, Deserialize};
use yew::prelude::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UnityCanvasTransform {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl UnityCanvasTransform {
    pub fn render_properties(&self) -> Html {
        html! {
            <div class="property-group">
                <h4>{"Unity Canvas Transform"}</h4>
                <div class="property-row">
                    <label>{"Position X"}</label>
                    <input type="number" value={self.x.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Position Y"}</label>
                    <input type="number" value={self.y.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Width"}</label>
                    <input type="number" value={self.width.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Height"}</label>
                    <input type="number" value={self.height.to_string()} />
                </div>
            </div>
        }
    }
} 