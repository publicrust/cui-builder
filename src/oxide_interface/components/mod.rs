pub mod cui_button_component;
pub mod cui_image_component;
pub mod cui_raw_image_component;
pub mod cui_rect_transform_component;
pub mod cui_text_component;
pub mod cui_needs_cursor_component;
pub mod cui_needs_keyboard_component;
pub mod component_type;

// -------------------------------------------------------------------
// Общий трейт для всех компонентов
// -------------------------------------------------------------------
use serde::Serialize;
use typetag;

#[typetag::serde(tag = "type")]
pub trait ICuiComponent: std::fmt::Debug + Send + Sync {
    fn component_type(&self) -> &'static str;
} 