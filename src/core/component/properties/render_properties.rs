use yew::prelude::*;
use crate::core::component::Component;

pub trait RenderProperties {
    fn render_properties_with_callback(&self, on_update: Callback<Component>) -> Html;
} 