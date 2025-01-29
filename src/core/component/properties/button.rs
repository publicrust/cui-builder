use yew::prelude::*;
use crate::oxide_interface::components::cui_button_component::CuiButtonComponent;
use crate::core::component::Component;
use super::RenderProperties;

impl RenderProperties for CuiButtonComponent {
    fn render_properties_with_callback(&self, on_update: Callback<Component>) -> Html {
        html! {
            <div class="component-properties">
                <h4>{"Button"}</h4>
                // TODO: Добавить редактирование свойств
            </div>
        }
    }
} 