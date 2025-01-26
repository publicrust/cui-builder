use serde::{Serialize, Deserialize};
use yew::prelude::*;
use crate::core::component::Component;

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

    pub fn render_properties_with_callback(&self, on_update: Callback<Component>) -> Html {
        let component = self.clone();
        
        html! {
            <div class="property-group">
                <h4>{"Button"}</h4>
                <div class="property-row">
                    <label>{"Text"}</label>
                    <input 
                        type="text"
                        value={component.text.clone()}
                        onchange={
                            let c = component.clone();
                            let on_update = on_update.clone();
                            Callback::from(move |e: Event| {
                                let mut c = c.clone();
                                if let Some(val) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                    c.text = val.value();
                                    on_update.emit(Component::Button(c));
                                }
                            })
                        }
                    />
                </div>
                <div class="property-row">
                    <label>{"Font Size"}</label>
                    <input 
                        type="number"
                        value={component.font_size.to_string()}
                        onchange={
                            let c = component.clone();
                            let on_update = on_update.clone();
                            Callback::from(move |e: Event| {
                                let mut c = c.clone();
                                if let Some(val) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                    if let Ok(size) = val.value().parse() {
                                        c.font_size = size;
                                        on_update.emit(Component::Button(c));
                                    }
                                }
                            })
                        }
                    />
                </div>
                <div class="property-row">
                    <label>{"Text Color"}</label>
                    <input 
                        type="text"
                        value={component.color.clone()}
                        onchange={
                            let c = component.clone();
                            let on_update = on_update.clone();
                            Callback::from(move |e: Event| {
                                let mut c = c.clone();
                                if let Some(val) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                    c.color = val.value();
                                    on_update.emit(Component::Button(c));
                                }
                            })
                        }
                    />
                </div>
                <div class="property-row">
                    <label>{"Background Color"}</label>
                    <input 
                        type="text"
                        value={component.background_color.clone()}
                        onchange={
                            let c = component.clone();
                            let on_update = on_update;
                            Callback::from(move |e: Event| {
                                let mut c = c.clone();
                                if let Some(val) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                    c.background_color = val.value();
                                    on_update.emit(Component::Button(c));
                                }
                            })
                        }
                    />
                </div>
            </div>
        }
    }
} 