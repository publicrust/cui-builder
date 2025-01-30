use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::oxide_interface::components::cui_raw_image_component::CuiRawImageComponent;
use crate::core::component::Component;
use super::RenderProperties;

impl RenderProperties for CuiRawImageComponent {
    fn render_properties_with_callback(&self, on_update: Callback<Component>) -> Html {
        let component = self.clone();
        
        let on_color_change = {
            let on_update = on_update.clone();
            let component = component.clone();
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let mut new_component = component.clone();
                    new_component.color = Some(input.value());
                    on_update.emit(Component::RawImage(new_component));
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
                    on_update.emit(Component::RawImage(new_component));
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
                    on_update.emit(Component::RawImage(new_component));
                }
            })
        };

        let on_url_change = {
            let on_update = on_update.clone();
            let component = component.clone();
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let mut new_component = component.clone();
                    new_component.url = Some(input.value());
                    on_update.emit(Component::RawImage(new_component));
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
                    on_update.emit(Component::RawImage(new_component));
                }
            })
        };

        let on_steamid_change = {
            let on_update = on_update.clone();
            let component = component.clone();
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let mut new_component = component.clone();
                    new_component.steamid = Some(input.value());
                    on_update.emit(Component::RawImage(new_component));
                }
            })
        };

        let on_fade_in_change = {
            let on_update = on_update;
            let component = component.clone();
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    if let Ok(value) = input.value().parse() {
                        let mut new_component = component.clone();
                        new_component.fade_in = Some(value);
                        on_update.emit(Component::RawImage(new_component));
                    }
                }
            })
        };

        html! {
            <div class="property-group">
                <h4>{"Raw Image"}</h4>
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
                    <label>{"URL"}</label>
                    <input 
                        type="text"
                        value={self.url.clone().unwrap_or_default()}
                        onchange={on_url_change}
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
                    <label>{"Steam ID"}</label>
                    <input 
                        type="text"
                        value={self.steamid.clone().unwrap_or_default()}
                        onchange={on_steamid_change}
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
            </div>
        }
    }

    fn render_properties(&self, container: &HtmlElement) {
        if let Some(color) = &self.color {
            container.set_attribute("data-color", color).ok();
        }
        if let Some(sprite) = &self.sprite {
            container.set_attribute("data-sprite", sprite).ok();
        }
        if let Some(material) = &self.material {
            container.set_attribute("data-material", material).ok();
        }
        if let Some(url) = &self.url {
            container.set_attribute("data-url", url).ok();
        }
        if let Some(png) = &self.png {
            container.set_attribute("data-png", png).ok();
        }
        if let Some(steamid) = &self.steamid {
            container.set_attribute("data-steamid", steamid).ok();
        }
        if let Some(fade_in) = &self.fade_in {
            container.set_attribute("data-fade-in", &fade_in.to_string()).ok();
        }
    }

    fn update_from_element(&mut self, element: &HtmlElement) {
        if let Some(color) = element.get_attribute("data-color") {
            self.color = Some(color);
        }
        if let Some(sprite) = element.get_attribute("data-sprite") {
            self.sprite = Some(sprite);
        }
        if let Some(material) = element.get_attribute("data-material") {
            self.material = Some(material);
        }
        if let Some(url) = element.get_attribute("data-url") {
            self.url = Some(url);
        }
        if let Some(png) = element.get_attribute("data-png") {
            self.png = Some(png);
        }
        if let Some(steamid) = element.get_attribute("data-steamid") {
            self.steamid = Some(steamid);
        }
        if let Some(fade_in) = element.get_attribute("data-fade-in") {
            if let Ok(value) = fade_in.parse() {
                self.fade_in = Some(value);
            }
        }
    }
} 