use serde::{Serialize, Deserialize};
use yew::prelude::*;
use crate::core::component::Component;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
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

    pub fn render_properties_with_callback(&self, on_update: Callback<Component>) -> Html {
        let component = self.clone();
        
        html! {
            <div class="property-group">
                <h4>{"Unity Canvas Transform"}</h4>
                <div class="property-row">
                    <label>{"Position X"}</label>
                    <input 
                        type="number"
                        value={component.x.to_string()}
                        onchange={
                            let c = component.clone();
                            let on_update = on_update.clone();
                            Callback::from(move |e: Event| {
                                let mut c = c.clone();
                                if let Some(val) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                    if let Ok(val) = val.value().parse::<f64>() {
                                        c.x = val;
                                        on_update.emit(Component::UnityCanvasTransform(c));
                                    }
                                }
                            })
                        }
                    />
                </div>
                <div class="property-row">
                    <label>{"Position Y"}</label>
                    <input 
                        type="number"
                        value={component.y.to_string()}
                        onchange={
                            let c = component.clone();
                            let on_update = on_update.clone();
                            Callback::from(move |e: Event| {
                                let mut c = c.clone();
                                if let Some(val) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                    if let Ok(val) = val.value().parse::<f64>() {
                                        c.y = val;
                                        on_update.emit(Component::UnityCanvasTransform(c));
                                    }
                                }
                            })
                        }
                    />
                </div>
                <div class="property-row">
                    <label>{"Width"}</label>
                    <input 
                        type="number"
                        min="0"
                        value={component.width.to_string()}
                        onchange={
                            let c = component.clone();
                            let on_update = on_update.clone();
                            Callback::from(move |e: Event| {
                                let mut c = c.clone();
                                if let Some(val) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                    if let Ok(val) = val.value().parse::<f64>() {
                                        c.width = val;
                                        on_update.emit(Component::UnityCanvasTransform(c));
                                    }
                                }
                            })
                        }
                    />
                </div>
                <div class="property-row">
                    <label>{"Height"}</label>
                    <input 
                        type="number"
                        min="0"
                        value={component.height.to_string()}
                        onchange={
                            let c = component.clone();
                            let on_update = on_update;
                            Callback::from(move |e: Event| {
                                let mut c = c.clone();
                                if let Some(val) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                    if let Ok(val) = val.value().parse::<f64>() {
                                        c.height = val;
                                        on_update.emit(Component::UnityCanvasTransform(c));
                                    }
                                }
                            })
                        }
                    />
                </div>
            </div>
        }
    }
} 