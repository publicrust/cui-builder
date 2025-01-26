use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Default for Vector2 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct Transform {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, PartialEq)]
pub struct RectTransform {
    pub anchor_min: Vector2,
    pub anchor_max: Vector2,
    pub offset_min: Vector2,
    pub offset_max: Vector2,
}

impl Default for RectTransform {
    fn default() -> Self {
        Self {
            anchor_min: Vector2 { x: 0.0, y: 0.0 },
            anchor_max: Vector2 { x: 1.0, y: 1.0 },
            offset_min: Vector2::default(),
            offset_max: Vector2::default(),
        }
    }
} 