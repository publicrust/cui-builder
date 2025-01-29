use serde::{Deserialize, Serialize};
use super::ICuiComponent;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct CuiNeedsCursorComponent;

#[typetag::serde]
impl ICuiComponent for CuiNeedsCursorComponent {
    fn component_type(&self) -> &'static str {
        "NeedsCursor"
    }
}

impl fmt::Display for CuiNeedsCursorComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CuiNeedsCursorComponent()")
    }
} 