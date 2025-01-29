use yew::prelude::*;
use crate::oxide_interface::components::cui_rect_transform_component::CuiRectTransformComponent;
use crate::core::component::Component;
use super::RenderProperties;

impl RenderProperties for CuiRectTransformComponent {
    fn render_properties_with_callback(&self, on_update: Callback<Component>) -> Html {
        html! {
            <div class="component-properties">
                <h4>{"RectTransform"}</h4>
                // TODO: Добавить редактирование свойств
            </div>
        }
    }
} 