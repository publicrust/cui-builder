use serde::{Serialize, Deserialize};
use yew::{Callback, Html};
use std::any::Any;
use downcast_rs::Downcast;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Component {
    RectTransform(super::rect_transform::RectTransform),
    Image(super::image::ImageComponent),
    RawImage(super::raw_image::RawImageComponent),
    Text(super::text::TextComponent),
    Button(super::button::ButtonComponent),
    NeedsCursor(super::needs_cursor::NeedsCursorComponent),
    NeedsKeyboard(super::needs_keyboard::NeedsKeyboardComponent),
}

impl Component {
    pub fn component_type(&self) -> &'static str {
        match self {
            Self::RectTransform(_) => "RectTransform",
            Self::Image(_) => "Image",
            Self::RawImage(_) => "RawImage",
            Self::Text(_) => "Text",
            Self::Button(_) => "Button",
            Self::NeedsCursor(_) => "NeedsCursor",
            Self::NeedsKeyboard(_) => "NeedsKeyboard",
        }
    }

    pub fn render_properties(&self, on_update: Callback<Box<dyn CuiComponent>>) -> Html {
        match self {
            Self::RectTransform(c) => c.render_properties(on_update),
            Self::Image(c) => c.render_properties(on_update),
            Self::RawImage(c) => c.render_properties(on_update),
            Self::Text(c) => c.render_properties(on_update),
            Self::Button(c) => c.render_properties(on_update),
            Self::NeedsCursor(c) => c.render_properties(on_update),
            Self::NeedsKeyboard(c) => c.render_properties(on_update),
        }
    }

    pub fn as_any(&self) -> &dyn Any {
        match self {
            Self::RectTransform(c) => c as &dyn Any,
            Self::Image(c) => c as &dyn Any,
            Self::RawImage(c) => c as &dyn Any,
            Self::Text(c) => c as &dyn Any,
            Self::Button(c) => c as &dyn Any,
            Self::NeedsCursor(c) => c as &dyn Any,
            Self::NeedsKeyboard(c) => c as &dyn Any,
        }
    }

    pub fn as_any_mut(&mut self) -> &mut dyn Any {
        match self {
            Self::RectTransform(c) => c as &mut dyn Any,
            Self::Image(c) => c as &mut dyn Any,
            Self::RawImage(c) => c as &mut dyn Any,
            Self::Text(c) => c as &mut dyn Any,
            Self::Button(c) => c as &mut dyn Any,
            Self::NeedsCursor(c) => c as &mut dyn Any,
            Self::NeedsKeyboard(c) => c as &mut dyn Any,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Point2D(pub f64, pub f64);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
    
    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
        Color { r, g, b, a: 255 }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

#[typetag::serde(tag = "type")]
pub trait CuiComponent: Any + Downcast + std::fmt::Debug + Send + Sync {
    fn component_type(&self) -> &'static str;
    fn render_properties(&self, on_update: Callback<Box<dyn CuiComponent>>) -> Html;
    
    fn as_any(&self) -> &dyn Any where Self: Sized {
        self as &dyn Any
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any where Self: Sized {
        self as &mut dyn Any
    }
    
    fn clone_box(&self) -> Box<dyn CuiComponent>;
}

downcast_rs::impl_downcast!(CuiComponent);

impl Clone for Box<dyn CuiComponent> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
} 