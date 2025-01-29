use yew::prelude::*;
use crate::oxide_interface::components::cui_text_component::CuiTextComponent;
use crate::core::component::Component;
use super::RenderProperties;

impl RenderProperties for CuiTextComponent {
    fn render_properties_with_callback(&self, on_update: Callback<Component>) -> Html {
        html! {
            <div class="component-properties">
                <h4>{"Text"}</h4>
                // TODO: Добавить редактирование свойств
            </div>
        }
    }
} 