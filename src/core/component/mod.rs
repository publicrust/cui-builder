mod rect;
mod image;
mod text;
mod button;
mod unity_canvas;

pub use rect::*;
pub use image::*;
pub use text::*;
pub use button::*;
pub use unity_canvas::*;

use serde::{Serialize, Deserialize};
use yew::prelude::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Component {
    RectTransform(RectTransformComponent),
    UnityCanvasTransform(UnityCanvasTransform),
    Image(ImageComponent),
    Text(TextComponent),
    Button(ButtonComponent),
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

    pub fn render_properties(&self) -> Html {
        match self {
            Component::RectTransform(c) => c.render_properties(),
            Component::UnityCanvasTransform(c) => c.render_properties(),
            Component::Image(c) => c.render_properties(),
            Component::Text(c) => c.render_properties(),
            Component::Button(c) => c.render_properties(),
        }
    }
} 