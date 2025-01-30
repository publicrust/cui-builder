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

pub mod oxide_interface;
pub mod components;
pub mod core;
pub mod styles;
pub mod models;

pub use oxide_interface::elements::{
    cui_element::CuiElement,
    cui_panel::CuiPanel,
    cui_button::CuiButton,
    cui_label::CuiLabel,
    ICuiElement,
};

// Re-export commonly used types
pub use oxide_interface::{
    CuiElementContainer,
    components::ICuiComponent,
};

// Re-export components
pub use components::{
    sidebar::Sidebar,
    canvas::InfiniteCanvas,
    properties::Properties,
}; 