pub mod sidebar;
pub mod properties;
pub mod infinite_canvas;
pub mod unity_canvas;
pub mod component;

use serde::{Deserialize, Serialize};
pub use component::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Anchor {
    pub x: f64,
    pub y: f64,
}

impl Default for Anchor {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Offset {
    pub x: f64,
    pub y: f64,
}

impl Default for Offset {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
}

// Позиция и размер на бесконечном листе
#[derive(Clone, PartialEq, Default)]
pub struct Transform {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

// Unity-подобный RectTransform для элементов внутри канваса
#[derive(Clone, PartialEq)]
pub struct RectTransform {
    pub anchor_min: Vector2,
    pub anchor_max: Vector2,
    pub offset_min: Vector2,
    pub offset_max: Vector2,
}

#[derive(Clone, PartialEq, Default)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

// Состояние перемещения по бесконечному листу
#[derive(Clone, PartialEq)]
pub struct WorkspaceState {
    pub offset_x: f64,
    pub offset_y: f64,
    pub scale: f64,
}

impl Default for WorkspaceState {
    fn default() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            scale: 1.0,
        }
    }
}

impl Default for RectTransform {
    fn default() -> Self {
        Self {
            anchor_min: Vector2 { x: 0.0, y: 0.0 },
            anchor_max: Vector2 { x: 1.0, y: 1.0 },
            offset_min: Vector2 { x: 0.0, y: 0.0 },
            offset_max: Vector2 { x: 0.0, y: 0.0 },
        }
    }
}

// Элементы на бесконечном листе (только канвасы)
#[derive(Clone, PartialEq)]
pub struct CanvasObject {
    pub id: String,
    pub name: String,
    pub transform: Transform,
    pub elements: Vec<Element>, // Элементы внутри канваса
}

// Элементы внутри Unity-канваса
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Element {
    pub id: String,
    pub name: String,
    pub element_type: ElementType,
    pub components: Vec<Component>,
    pub children: Vec<Element>,
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id 
            && self.name == other.name 
            && self.element_type == other.element_type
            && self.children == other.children
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ElementType {
    UnityCanvas,
    Panel,
    Text,
    Button,
}

impl Default for ElementType {
    fn default() -> Self {
        Self::UnityCanvas
    }
} 