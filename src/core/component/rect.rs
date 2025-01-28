use serde::{Serialize, Deserialize};
use yew::prelude::*;
use super::Component;
use web_sys::{HtmlInputElement, Event};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RectTransformComponent {
    pub position: (f64, f64),
    pub size: (f64, f64),
    pub rotation: f64,
}

impl Default for RectTransformComponent {
    fn default() -> Self {
        Self {
            position: (0.0, 0.0),
            size: (100.0, 100.0),
            rotation: 0.0,
        }
    }
}

impl RectTransformComponent {
    pub fn render_properties_with_callback(&self, on_update: Callback<Component>) -> Html {
        let position = self.position;
        let size = self.size;
        let rotation = self.rotation;

        html! {
            <div class="properties-group">
                <h3>{"Transform"}</h3>
                <div class="property">
                    <label>{"Position"}</label>
                    <input 
                        type="number" 
                        value={position.0.to_string()} 
                        onchange={{
                            let on_update = on_update.clone();
                            let component = self.clone();
                            Callback::from(move |e: Event| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    if let Ok(value) = input.value().parse::<f64>() {
                                        let mut new_transform = component.clone();
                                        new_transform.position.0 = value;
                                        on_update.emit(Component::RectTransform(new_transform));
                                    }
                                }
                            })
                        }}
                    />
                    <input 
                        type="number" 
                        value={position.1.to_string()} 
                        onchange={{
                            let on_update = on_update.clone();
                            let component = self.clone();
                            Callback::from(move |e: Event| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    if let Ok(value) = input.value().parse::<f64>() {
                                        let mut new_transform = component.clone();
                                        new_transform.position.1 = value;
                                        on_update.emit(Component::RectTransform(new_transform));
                                    }
                                }
                            })
                        }}
                    />
                </div>
                <div class="property">
                    <label>{"Size"}</label>
                    <input 
                        type="number" 
                        value={size.0.to_string()} 
                        onchange={{
                            let on_update = on_update.clone();
                            let component = self.clone();
                            Callback::from(move |e: Event| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    if let Ok(value) = input.value().parse::<f64>() {
                                        let mut new_transform = component.clone();
                                        new_transform.size.0 = value;
                                        on_update.emit(Component::RectTransform(new_transform));
                                    }
                                }
                            })
                        }}
                    />
                    <input 
                        type="number" 
                        value={size.1.to_string()} 
                        onchange={{
                            let on_update = on_update.clone();
                            let component = self.clone();
                            Callback::from(move |e: Event| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    if let Ok(value) = input.value().parse::<f64>() {
                                        let mut new_transform = component.clone();
                                        new_transform.size.1 = value;
                                        on_update.emit(Component::RectTransform(new_transform));
                                    }
                                }
                            })
                        }}
                    />
                </div>
                <div class="property">
                    <label>{"Rotation"}</label>
                    <input 
                        type="number" 
                        value={rotation.to_string()} 
                        onchange={{
                            let on_update = on_update;
                            let component = self.clone();
                            Callback::from(move |e: Event| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    if let Ok(value) = input.value().parse::<f64>() {
                                        let mut new_transform = component.clone();
                                        new_transform.rotation = value;
                                        on_update.emit(Component::RectTransform(new_transform));
                                    }
                                }
                            })
                        }}
                    />
                </div>
            </div>
        }
    }
} 