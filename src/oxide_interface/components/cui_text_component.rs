use serde::{Deserialize, Serialize};
use super::ICuiComponent;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CuiTextComponent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_in: Option<f32>,
}

impl Default for CuiTextComponent {
    fn default() -> Self {
        Self {
            text: None,
            font_size: None,
            align: None,
            color: None,
            fade_in: None,
        }
    }
}

#[typetag::serde]
impl ICuiComponent for CuiTextComponent {
    fn component_type(&self) -> &'static str {
        "UnityEngine.UI.Text"
    }
}

impl fmt::Display for CuiTextComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CuiTextComponent()")
    }
} 