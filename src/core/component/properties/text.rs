use yew::prelude::*;
use crate::oxide_interface::components::cui_text_component::CuiTextComponent;
use crate::core::component::Component;
use super::RenderProperties;

impl RenderProperties for CuiTextComponent {
    fn render_properties_with_callback(&self, on_update: Callback<Component>) -> Html {
        let component = self.clone();
        
        let on_text_change = {
            let on_update = on_update.clone();
            let component = component.clone();
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let mut new_component = component.clone();
                    new_component.text = Some(input.value());
                    on_update.emit(Component::Text(new_component));
                }
            })
        };

        let on_color_change = {
            let on_update = on_update.clone();
            let component = component.clone();
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let mut new_component = component.clone();
                    new_component.color = Some(input.value());
                    on_update.emit(Component::Text(new_component));
                }
            })
        };

        let on_font_size_change = {
            let on_update = on_update.clone();
            let component = component.clone();
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    if let Ok(value) = input.value().parse() {
                        let mut new_component = component.clone();
                        new_component.font_size = Some(value);
                        on_update.emit(Component::Text(new_component));
                    }
                }
            })
        };

        let on_align_change = {
            let on_update = on_update.clone();
            let component = component.clone();
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let mut new_component = component.clone();
                    new_component.align = Some(input.value());
                    on_update.emit(Component::Text(new_component));
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
                        on_update.emit(Component::Text(new_component));
                    }
                }
            })
        };

        html! {
            <div class="property-group">
                <h4>{"Text"}</h4>
                <div class="property-row">
                    <label>{"Text"}</label>
                    <input 
                        type="text"
                        value={self.text.clone().unwrap_or_default()}
                        onchange={on_text_change}
                    />
                </div>
                <div class="property-row">
                    <label>{"Color"}</label>
                    <input 
                        type="text"
                        value={self.color.clone().unwrap_or_default()}
                        onchange={on_color_change}
                    />
                </div>
                <div class="property-row">
                    <label>{"Font Size"}</label>
                    <input 
                        type="number"
                        step="1"
                        value={self.font_size.map(|v| v.to_string()).unwrap_or_default()}
                        onchange={on_font_size_change}
                    />
                </div>
                <div class="property-row">
                    <label>{"Align"}</label>
                    <input 
                        type="text"
                        value={self.align.clone().unwrap_or_default()}
                        onchange={on_align_change}
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
} 