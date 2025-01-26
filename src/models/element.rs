use serde::{Deserialize, Serialize};
use crate::core::component::Component;
use std::fmt;

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

impl fmt::Display for ElementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElementType::UnityCanvas => write!(f, "Unity Canvas"),
            ElementType::Panel => write!(f, "Panel"),
            ElementType::Text => write!(f, "Text"),
            ElementType::Button => write!(f, "Button"),
        }
    }
}

impl Default for ElementType {
    fn default() -> Self {
        Self::UnityCanvas
    }
} 