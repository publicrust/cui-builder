use yew::prelude::*;
use crate::oxide_interface::components::cui_button_component::CuiButtonComponent;
use crate::core::component::Component;
use super::RenderProperties;

impl RenderProperties for CuiButtonComponent {
    fn render_properties_with_callback(&self, on_update: Callback<Component>) -> Html {
        let component = self.clone();
        
        let on_color_change = {
            let on_update = on_update.clone();
            let component = component.clone();
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let mut new_component = component.clone();
                    new_component.color = Some(input.value());
                    on_update.emit(Component::Button(new_component));
                }
            })
        };

        let on_command_change = {
            let on_update = on_update.clone();
            let component = component.clone();
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let mut new_component = component.clone();
                    new_component.command = Some(input.value());
                    on_update.emit(Component::Button(new_component));
                }
            })
        };

        let on_close_change = {
            let on_update = on_update.clone();
            let component = component.clone();
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let mut new_component = component.clone();
                    new_component.close = Some(input.value());
                    on_update.emit(Component::Button(new_component));
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
                        new_component.fade_in = value;
                        on_update.emit(Component::Button(new_component));
                    }
                }
            })
        };

        html! {
            <div class="property-group">
                <h4>{"Button"}</h4>
                <div class="property-row">
                    <label>{"Color"}</label>
                    <input 
                        type="text"
                        value={self.color.clone().unwrap_or_default()}
                        onchange={on_color_change}
                    />
                </div>
                <div class="property-row">
                    <label>{"Command"}</label>
                    <input 
                        type="text"
                        value={self.command.clone().unwrap_or_default()}
                        onchange={on_command_change}
                    />
                </div>
                <div class="property-row">
                    <label>{"Close"}</label>
                    <input 
                        type="text"
                        value={self.close.clone().unwrap_or_default()}
                        onchange={on_close_change}
                    />
                </div>
                <div class="property-row">
                    <label>{"Fade In"}</label>
                    <input 
                        type="number"
                        step="0.1"
                        value={self.fade_in.to_string()}
                        onchange={on_fade_in_change}
                    />
                </div>
            </div>
        }
    }
} 