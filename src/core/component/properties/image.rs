use yew::prelude::*;
use crate::oxide_interface::components::cui_image_component::CuiImageComponent;
use crate::core::component::Component;
use super::RenderProperties;

impl RenderProperties for CuiImageComponent {
    fn render_properties_with_callback(&self, on_update: Callback<Component>) -> Html {
        html! {
            <div class="component-properties">
                <h4>{"Image"}</h4>
                // TODO: Добавить редактирование свойств
            </div>
        }
    }
} 