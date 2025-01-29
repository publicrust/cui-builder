mod unity_canvas;
mod properties;

pub use unity_canvas::*;
pub use properties::RenderProperties;
pub use crate::oxide_interface::components::{
    cui_rect_transform_component::CuiRectTransformComponent,
    cui_image_component::CuiImageComponent,
    cui_text_component::CuiTextComponent,
    cui_button_component::CuiButtonComponent,
    ICuiComponent,
    component_type::ComponentType,
};

use serde::{Serialize, Deserialize};
use yew::prelude::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Component {
    RectTransform(CuiRectTransformComponent),
    UnityCanvasTransform(UnityCanvasTransform),
    Image(CuiImageComponent),
    Text(CuiTextComponent),
    Button(CuiButtonComponent),
}

impl Component {
    pub fn component_type(&self) -> &'static str {
        match self {
            Component::RectTransform(_) => "RectTransform",
            Component::UnityCanvasTransform(_) => "UnityCanvasTransform",
            Component::Image(_) => "Image",
            Component::Text(_) => "Text",
            Component::Button(_) => "Button",
        }
    }

    pub fn render_properties_with_callback(&self, on_update: Callback<Component>) -> Html {
        match self {
            Component::RectTransform(c) => c.render_properties_with_callback(on_update),
            Component::UnityCanvasTransform(c) => c.render_properties_with_callback(on_update),
            Component::Image(c) => c.render_properties_with_callback(on_update),
            Component::Text(c) => c.render_properties_with_callback(on_update),
            Component::Button(c) => c.render_properties_with_callback(on_update),
        }
    }
} 