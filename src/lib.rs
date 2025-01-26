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

pub mod models;
pub mod core;
pub mod components;
pub mod styles;

// Re-export commonly used types
pub use models::{Element, ElementType};
pub use core::component::Component; 