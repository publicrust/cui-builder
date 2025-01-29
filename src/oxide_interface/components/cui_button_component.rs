use serde::{Deserialize, Serialize};
use super::ICuiComponent;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct CuiButtonComponent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>, // В C# есть поля на наклик и т.д.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    pub fade_in: f32,
}

#[typetag::serde]
impl ICuiComponent for CuiButtonComponent {
    fn component_type(&self) -> &'static str {
        "UnityEngine.UI.Button"
    }
}

impl fmt::Display for CuiButtonComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CuiButtonComponent()")
    }
} 