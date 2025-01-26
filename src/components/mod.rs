pub mod sidebar;
pub mod properties;
pub mod infinite_canvas;
pub mod unity_canvas;

use serde::{Deserialize, Serialize};

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
    pub anchor_min: Vector2,  // От 0 до 1, определяет привязку к родителю (левый нижний угол)
    pub anchor_max: Vector2,  // От 0 до 1, определяет привязку к родителю (правый верхний угол)
    pub offset_min: Vector2,  // Отступ от точки привязки в пикселях (левый нижний угол)
    pub offset_max: Vector2,  // Отступ от точки привязки в пикселях (правый верхний угол)
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
            anchor_min: Vector2 { x: 0.5, y: 0.5 },
            anchor_max: Vector2 { x: 0.5, y: 0.5 },
            offset_min: Vector2 { x: -50.0, y: -50.0 },
            offset_max: Vector2 { x: 50.0, y: 50.0 },
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
#[derive(Clone, PartialEq)]
pub struct Element {
    pub id: String,
    pub name: String,
    pub element_type: ElementType,
    pub rect_transform: RectTransform,
    pub children: Vec<Element>,
}

#[derive(Clone, PartialEq)]
pub enum ElementType {
    Container,  // Контейнер внутри канваса
    Text,       // Текстовый элемент
    Image,      // Изображение
    Button,     // Кнопка
}

impl Default for ElementType {
    fn default() -> Self {
        Self::Container
    }
} 