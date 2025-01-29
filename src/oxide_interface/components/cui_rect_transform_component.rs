use serde::{Deserialize, Serialize};
use super::ICuiComponent;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CuiRectTransform {
    pub anchormin: String,
    pub anchormax: String,
    pub offsetmin: String,
    pub offsetmax: String,
}

impl Default for CuiRectTransform {
    fn default() -> Self {
        Self {
            anchormin: "0 0".to_string(),
            anchormax: "1 1".to_string(),
            offsetmin: "0 0".to_string(),
            offsetmax: "0 0".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CuiRectTransformComponent {
    #[serde(flatten)]
    pub base: CuiRectTransform,
}

impl Default for CuiRectTransformComponent {
    fn default() -> Self {
        Self {
            base: CuiRectTransform::default()
        }
    }
}

#[typetag::serde]
impl ICuiComponent for CuiRectTransformComponent {
    fn component_type(&self) -> &'static str {
        "RectTransform"
    }
}

impl fmt::Display for CuiRectTransformComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CuiRectTransformComponent()")
    }
} 