use serde::{Deserialize, Serialize};
use super::ICuiComponent;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CuiNeedsKeyboardComponent;

#[typetag::serde]
impl ICuiComponent for CuiNeedsKeyboardComponent {
    fn component_type(&self) -> &'static str {
        "NeedsKeyboard"
    }
}

impl fmt::Display for CuiNeedsKeyboardComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CuiNeedsKeyboardComponent()")
    }
} 