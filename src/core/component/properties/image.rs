use yew::prelude::*;
use web_sys::{HtmlInputElement, HtmlElement};
use wasm_bindgen::JsCast;
use crate::oxide_interface::components::cui_image_component::CuiImageComponent;
use crate::core::component::Component;
use super::RenderProperties;

impl RenderProperties for CuiImageComponent {
    fn render_properties_with_callback(&self, on_update: Callback<Component>) -> Html {
        let component = self.clone();
        
        let on_color_change = {
            let on_update = on_update.clone();
            let component = component.clone();
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let mut new_component = component.clone();
                    new_component.color = Some(input.value());
                    on_update.emit(Component::Image(new_component));
                }
            })
        };

        let on_sprite_change = {
            let on_update = on_update.clone();
            let component = component.clone();
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let mut new_component = component.clone();
                    new_component.sprite = Some(input.value());
                    on_update.emit(Component::Image(new_component));
                }
            })
        };

        let on_material_change = {
            let on_update = on_update.clone();
            let component = component.clone();
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let mut new_component = component.clone();
                    new_component.material = Some(input.value());
                    on_update.emit(Component::Image(new_component));
                }
            })
        };

        let on_image_type_change = {
            let on_update = on_update.clone();
            let component = component.clone();
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let mut new_component = component.clone();
                    new_component.image_type = Some(input.value());
                    on_update.emit(Component::Image(new_component));
                }
            })
        };

        let on_png_change = {
            let on_update = on_update.clone();
            let component = component.clone();
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let mut new_component = component.clone();
                    new_component.png = Some(input.value());
                    on_update.emit(Component::Image(new_component));
                }
            })
        };

        let on_fade_in_change = {
            let on_update = on_update.clone();
            let component = component.clone();
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    if let Ok(value) = input.value().parse::<f32>() {
                        let mut new_component = component.clone();
                        new_component.fade_in = Some(value);
                        on_update.emit(Component::Image(new_component));
                    }
                }
            })
        };

        let on_itemid_change = {
            let on_update = on_update.clone();
            let component = component.clone();
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    if let Ok(value) = input.value().parse::<i32>() {
                        let mut new_component = component.clone();
                        new_component.itemid = Some(value);
                        on_update.emit(Component::Image(new_component));
                    }
                }
            })
        };

        let on_skinid_change = {
            let on_update = on_update;
            let component = component.clone();
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    if let Ok(value) = input.value().parse::<u64>() {
                        let mut new_component = component.clone();
                        new_component.skinid = Some(value);
                        on_update.emit(Component::Image(new_component));
                    }
                }
            })
        };

        html! {
            <div class="property-group">
                <h4>{"Image"}</h4>
                <div class="property-row">
                    <label>{"Color"}</label>
                    <input 
                        type="text"
                        value={self.color.clone().unwrap_or_default()}
                        onchange={on_color_change}
                    />
                </div>
                <div class="property-row">
                    <label>{"Sprite"}</label>
                    <input 
                        type="text"
                        value={self.sprite.clone().unwrap_or_default()}
                        onchange={on_sprite_change}
                    />
                </div>
                <div class="property-row">
                    <label>{"Material"}</label>
                    <input 
                        type="text"
                        value={self.material.clone().unwrap_or_default()}
                        onchange={on_material_change}
                    />
                </div>
                <div class="property-row">
                    <label>{"Image Type"}</label>
                    <input 
                        type="text"
                        value={self.image_type.clone().unwrap_or_default()}
                        onchange={on_image_type_change}
                    />
                </div>
                <div class="property-row">
                    <label>{"PNG"}</label>
                    <input 
                        type="text"
                        value={self.png.clone().unwrap_or_default()}
                        onchange={on_png_change}
                    />
                </div>
                <div class="property-row">
                    <label>{"Fade In"}</label>
                    <input 
                        type="number"
                        step="0.1"
                        value={self.fade_in.map(|v| v.to_string()).unwrap_or_default()}
                        onchange={on_fade_in_change}
                    />
                </div>
                <div class="property-row">
                    <label>{"Item ID"}</label>
                    <input 
                        type="number"
                        value={self.itemid.map(|v| v.to_string()).unwrap_or_default()}
                        onchange={on_itemid_change}
                    />
                </div>
                <div class="property-row">
                    <label>{"Skin ID"}</label>
                    <input 
                        type="number"
                        value={self.skinid.map(|v| v.to_string()).unwrap_or_default()}
                        onchange={on_skinid_change}
                    />
                </div>
            </div>
        }
    }

    fn render_properties(&self, container: &HtmlElement) {
        if let Some(sprite) = &self.sprite {
            container.set_attribute("data-sprite", sprite).ok();
        }
        if let Some(material) = &self.material {
            container.set_attribute("data-material", material).ok();
        }
    }

    fn update_from_element(&mut self, element: &HtmlElement) {
        if let Some(sprite) = element.get_attribute("data-sprite") {
            self.sprite = Some(sprite);
        }
        if let Some(material) = element.get_attribute("data-material") {
            self.material = Some(material);
        }
    }
} 