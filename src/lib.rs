#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::unreachable)]
#![deny(clippy::print_stdout)]
#![deny(clippy::dbg_macro)]
#![deny(clippy::use_debug)]
#![deny(clippy::disallowed_types)]

extern crate downcast_rs;

pub mod shared;
pub mod entities;
pub mod components;
pub mod features;
pub mod widgets;

// Re-export commonly used types
pub use entities::cui_element::model::CuiElement;
pub use entities::cui_container::model::CuiContainer;
pub use shared::lib::component::Component;
pub use shared::lib::types::{Point, Size, Rect, Color};

// Re-export features
pub use features::canvas::Canvas;
pub use features::properties::PropertiesPanel;
pub use features::element_tree::ElementTree;

pub use entities::element::Element; 