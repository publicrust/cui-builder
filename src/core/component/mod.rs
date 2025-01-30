pub mod unity_canvas;
mod properties;

pub use unity_canvas::*;
pub use properties::*;
pub use crate::oxide_interface::components::{
    cui_rect_transform_component::CuiRectTransformComponent,
    cui_image_component::CuiImageComponent,
    cui_text_component::CuiTextComponent,
    cui_button_component::CuiButtonComponent,
    cui_raw_image_component::CuiRawImageComponent,
    cui_needs_cursor_component::CuiNeedsCursorComponent,
    cui_needs_keyboard_component::CuiNeedsKeyboardComponent,
    ICuiComponent,
    component_type::ComponentType,
};

use yew::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Component {
    RectTransform(CuiRectTransformComponent),
    Image(CuiImageComponent),
    Button(CuiButtonComponent),
    Text(CuiTextComponent),
    RawImage(CuiRawImageComponent),
    NeedsCursor(CuiNeedsCursorComponent),
    NeedsKeyboard(CuiNeedsKeyboardComponent),
    UnityCanvasTransform(UnityCanvasTransform),
}

impl Component {
    pub fn component_type(&self) -> &'static str {
        match self {
            Component::RectTransform(_) => "RectTransform",
            Component::Image(_) => "Image",
            Component::Button(_) => "Button",
            Component::Text(_) => "Text",
            Component::RawImage(_) => "RawImage",
            Component::NeedsCursor(_) => "NeedsCursor",
            Component::NeedsKeyboard(_) => "NeedsKeyboard",
            Component::UnityCanvasTransform(_) => "UnityCanvasTransform",
        }
    }
} 