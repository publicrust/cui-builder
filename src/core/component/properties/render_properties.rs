use yew::prelude::*;
use crate::core::component::Component;
use web_sys::HtmlElement;

pub trait RenderProperties {
    fn render_properties_with_callback(&self, on_update: Callback<Component>) -> Html;
    fn render_properties(&self, container: &HtmlElement);
    fn update_from_element(&mut self, element: &HtmlElement);
} 