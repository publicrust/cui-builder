use serde::{Deserialize, Serialize};
use super::ICuiComponent;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CuiImageComponent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprite: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "imagetype", skip_serializing_if = "Option::is_none")]
    pub image_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub png: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_in: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub itemid: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skinid: Option<u64>,
}

impl Default for CuiImageComponent {
    fn default() -> Self {
        Self {
            sprite: None,
            material: None,
            color: None,
            image_type: None,
            png: None,
            fade_in: None,
            itemid: None,
            skinid: None,
        }
    }
}

#[typetag::serde]
impl ICuiComponent for CuiImageComponent {
    fn component_type(&self) -> &'static str {
        "UnityEngine.UI.Image"
    }
}

impl fmt::Display for CuiImageComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CuiImageComponent()")
    }
} 