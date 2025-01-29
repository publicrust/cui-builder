use serde::{Deserialize, Serialize};
use super::ICuiComponent;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CuiRawImageComponent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprite: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub png: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steamid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_in: Option<f32>,
}

impl Default for CuiRawImageComponent {
    fn default() -> Self {
        Self {
            sprite: None,
            color: None,
            material: None,
            url: None,
            png: None,
            steamid: None,
            fade_in: None,
        }
    }
}

#[typetag::serde]
impl ICuiComponent for CuiRawImageComponent {
    fn component_type(&self) -> &'static str {
        "UnityEngine.UI.RawImage"
    }
}

impl fmt::Display for CuiRawImageComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CuiRawImageComponent()")
    }
} 