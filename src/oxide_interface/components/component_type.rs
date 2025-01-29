use serde::{Deserialize, Serialize};
use super::{
    cui_button_component::CuiButtonComponent,
    cui_image_component::CuiImageComponent,
    cui_raw_image_component::CuiRawImageComponent,
    cui_rect_transform_component::CuiRectTransformComponent,
    cui_text_component::CuiTextComponent,
    cui_needs_cursor_component::CuiNeedsCursorComponent,
    cui_needs_keyboard_component::CuiNeedsKeyboardComponent,
    ICuiComponent,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ComponentType {
    #[serde(rename = "UnityEngine.UI.Button")]
    Button(CuiButtonComponent),
    #[serde(rename = "UnityEngine.UI.Image")]
    Image(CuiImageComponent),
    #[serde(rename = "UnityEngine.UI.RawImage")]
    RawImage(CuiRawImageComponent),
    #[serde(rename = "RectTransform")]
    RectTransform(CuiRectTransformComponent),
    #[serde(rename = "UnityEngine.UI.Text")]
    Text(CuiTextComponent),
    #[serde(rename = "NeedsCursor")]
    NeedsCursor(CuiNeedsCursorComponent),
    #[serde(rename = "NeedsKeyboard")]
    NeedsKeyboard(CuiNeedsKeyboardComponent),
}

#[typetag::serde]
impl ICuiComponent for ComponentType {
    fn component_type(&self) -> &'static str {
        match self {
            Self::Button(_) => "UnityEngine.UI.Button",
            Self::Image(_) => "UnityEngine.UI.Image",
            Self::RawImage(_) => "UnityEngine.UI.RawImage",
            Self::RectTransform(_) => "RectTransform",
            Self::Text(_) => "UnityEngine.UI.Text",
            Self::NeedsCursor(_) => "NeedsCursor",
            Self::NeedsKeyboard(_) => "NeedsKeyboard",
        }
    }
} 