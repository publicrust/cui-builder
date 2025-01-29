use yew::prelude::*;
use crate::core::component::{Component, UnityCanvasTransform};
use super::RenderProperties;

impl RenderProperties for UnityCanvasTransform {
    fn render_properties_with_callback(&self, on_update: Callback<Component>) -> Html {
        html! {
            <div class="component-properties">
                <h4>{"UnityCanvasTransform"}</h4>
                // TODO: Добавить редактирование свойств
            </div>
        }
    }
} 