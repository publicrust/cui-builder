pub mod button;
pub mod label;
pub mod panel;
pub mod image;
pub mod raw_image;
pub mod rect_transform;
pub mod text;
pub mod needs_cursor;
pub mod needs_keyboard;

// -------------------------------------------------------------------
// Общий трейт для всех компонентов
// -------------------------------------------------------------------
use serde::Serialize;
use typetag;

#[typetag::serde(tag = "type")]
pub trait ICuiComponent: std::fmt::Debug + Send + Sync {
    fn component_type(&self) -> &'static str;
} 