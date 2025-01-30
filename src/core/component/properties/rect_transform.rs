use yew::prelude::*;
use web_sys::{HtmlInputElement, HtmlElement};
use wasm_bindgen::JsCast;
use crate::core::component::Component;
use crate::oxide_interface::components::cui_rect_transform_component::CuiRectTransformComponent;
use super::RenderProperties;

impl RenderProperties for CuiRectTransformComponent {
    fn render_properties_with_callback(&self, on_update: Callback<Component>) -> Html {
        let component = self.clone();
        
        let on_anchormin_change = {
            let on_update = on_update.clone();
            let component = component.clone();
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let mut new_component = component.clone();
                    new_component.base.anchormin = input.value();
                    on_update.emit(Component::RectTransform(new_component));
                }
            })
        };

        let on_anchormax_change = {
            let on_update = on_update.clone();
            let component = component.clone();
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let mut new_component = component.clone();
                    new_component.base.anchormax = input.value();
                    on_update.emit(Component::RectTransform(new_component));
                }
            })
        };

        let on_offsetmin_change = {
            let on_update = on_update.clone();
            let component = component.clone();
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let mut new_component = component.clone();
                    new_component.base.offsetmin = input.value();
                    on_update.emit(Component::RectTransform(new_component));
                }
            })
        };

        let on_offsetmax_change = {
            let on_update = on_update;
            let component = component.clone();
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let mut new_component = component.clone();
                    new_component.base.offsetmax = input.value();
                    on_update.emit(Component::RectTransform(new_component));
                }
            })
        };

        html! {
            <div class="property-group">
                <h4>{"RectTransform"}</h4>
                <div class="property-row">
                    <label>{"Anchor Min"}</label>
                    <input 
                        type="text"
                        value={self.base.anchormin.clone()}
                        onchange={on_anchormin_change}
                    />
                </div>
                <div class="property-row">
                    <label>{"Anchor Max"}</label>
                    <input 
                        type="text"
                        value={self.base.anchormax.clone()}
                        onchange={on_anchormax_change}
                    />
                </div>
                <div class="property-row">
                    <label>{"Offset Min"}</label>
                    <input 
                        type="text"
                        value={self.base.offsetmin.clone()}
                        onchange={on_offsetmin_change}
                    />
                </div>
                <div class="property-row">
                    <label>{"Offset Max"}</label>
                    <input 
                        type="text"
                        value={self.base.offsetmax.clone()}
                        onchange={on_offsetmax_change}
                    />
                </div>
            </div>
        }
    }

    fn render_properties(&self, container: &HtmlElement) {
        container.set_attribute("data-anchormin", &self.base.anchormin).ok();
        container.set_attribute("data-anchormax", &self.base.anchormax).ok();
        container.set_attribute("data-offsetmin", &self.base.offsetmin).ok();
        container.set_attribute("data-offsetmax", &self.base.offsetmax).ok();
    }

    fn update_from_element(&mut self, element: &HtmlElement) {
        if let Some(anchormin) = element.get_attribute("data-anchormin") {
            self.base.anchormin = anchormin;
        }
        if let Some(anchormax) = element.get_attribute("data-anchormax") {
            self.base.anchormax = anchormax;
        }
        if let Some(offsetmin) = element.get_attribute("data-offsetmin") {
            self.base.offsetmin = offsetmin;
        }
        if let Some(offsetmax) = element.get_attribute("data-offsetmax") {
            self.base.offsetmax = offsetmax;
        }
    }
} 