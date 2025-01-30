use yew::prelude::*;
use web_sys::{HtmlInputElement, HtmlElement};
use wasm_bindgen::JsCast;
use crate::core::component::Component;
use crate::oxide_interface::components::cui_button_component::CuiButtonComponent;
use super::RenderProperties;

impl RenderProperties for CuiButtonComponent {
    fn render_properties_with_callback(&self, on_update: Callback<Component>) -> Html {
        let component = self.clone();
        
        let on_color_change = {
            let on_update = on_update.clone();
            let component = component.clone();
            Callback::from(move |e: Event| {
                let input: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
                let mut new_component = component.clone();
                new_component.color = Some(input.value());
                on_update.emit(Component::Button(new_component));
            })
        };

        let on_command_change = {
            let on_update = on_update.clone();
            let component = component.clone();
            Callback::from(move |e: Event| {
                let input: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
                let mut new_component = component.clone();
                new_component.command = Some(input.value());
                on_update.emit(Component::Button(new_component));
            })
        };

        html! {
            <div class="component-properties">
                <div class="property">
                    <label>{"Text:"}</label>
                    <input type="text" value={component.command.clone().unwrap_or_default()} onchange={on_command_change}/>
                </div>
                <div class="property">
                    <label>{"Color:"}</label>
                    <input type="color" value={component.color.clone().unwrap_or_default()} onchange={on_color_change}/>
                </div>
            </div>
        }
    }

    fn render_properties(&self, container: &HtmlElement) {
        if let Some(color) = &self.color {
            container.set_attribute("data-color", color).ok();
        }
        if let Some(command) = &self.command {
            container.set_attribute("data-command", command).ok();
        }
    }

    fn update_from_element(&mut self, element: &HtmlElement) {
        if let Some(color) = element.get_attribute("data-color") {
            self.color = Some(color);
        }
        if let Some(command) = element.get_attribute("data-command") {
            self.command = Some(command);
        }
    }
} 