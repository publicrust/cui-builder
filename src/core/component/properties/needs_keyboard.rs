use yew::prelude::*;
use web_sys::HtmlElement;
use crate::oxide_interface::components::cui_needs_keyboard_component::CuiNeedsKeyboardComponent;
use crate::core::component::Component;
use super::RenderProperties;

impl RenderProperties for CuiNeedsKeyboardComponent {
    fn render_properties_with_callback(&self, on_update: Callback<Component>) -> Html {
        html! {
            <div class="property-group">
                <h4>{"Needs Keyboard"}</h4>
                <div class="property-row">
                    <label>{"Requires keyboard interaction"}</label>
                </div>
            </div>
        }
    }

    fn render_properties(&self, _container: &HtmlElement) {
        // No properties to render
    }

    fn update_from_element(&mut self, _element: &HtmlElement) {
        // No properties to update
    }
} 