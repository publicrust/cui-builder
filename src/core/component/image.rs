use serde::{Serialize, Deserialize};
use yew::prelude::*;
use crate::core::component::Component;

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

    pub fn render_properties_with_callback(&self, on_update: Callback<Component>) -> Html {
        let component = self.clone();
        
        html! {
            <div class="property-group">
                <h4>{"Image"}</h4>
                <div class="property-row">
                    <label>{"Sprite"}</label>
                    <input 
                        type="text"
                        value={component.sprite.clone().unwrap_or_default()}
                        onchange={
                            let c = component.clone();
                            let on_update = on_update.clone();
                            Callback::from(move |e: Event| {
                                let mut c = c.clone();
                                if let Some(val) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                    let value = val.value();
                                    c.sprite = if value.is_empty() { None } else { Some(value) };
                                    on_update.emit(Component::Image(c));
                                }
                            })
                        }
                    />
                </div>
                <div class="property-row">
                    <label>{"Color"}</label>
                    <input 
                        type="text"
                        value={component.color.clone().unwrap_or_default()}
                        onchange={
                            let c = component.clone();
                            let on_update = on_update.clone();
                            Callback::from(move |e: Event| {
                                let mut c = c.clone();
                                if let Some(val) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                    let value = val.value();
                                    c.color = if value.is_empty() { None } else { Some(value) };
                                    on_update.emit(Component::Image(c));
                                }
                            })
                        }
                    />
                </div>
                <div class="property-row">
                    <label>{"Material"}</label>
                    <input 
                        type="text"
                        value={component.material.clone().unwrap_or_default()}
                        onchange={
                            let c = component.clone();
                            let on_update = on_update;
                            Callback::from(move |e: Event| {
                                let mut c = c.clone();
                                if let Some(val) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                    let value = val.value();
                                    c.material = if value.is_empty() { None } else { Some(value) };
                                    on_update.emit(Component::Image(c));
                                }
                            })
                        }
                    />
                </div>
            </div>
        }
    }
} 