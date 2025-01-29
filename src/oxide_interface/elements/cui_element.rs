use serde::{Serialize, Deserialize};
use crate::oxide_interface::components::{ICuiComponent, component_type::ComponentType};
use super::ICuiElement;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CuiElement {
    pub name: String,
    pub parent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroy_ui: Option<String>,
    pub components: Vec<ComponentType>,
    pub fade_out: f32,
}

impl CuiElement {
    pub fn new(name: String, parent: String, components: Vec<ComponentType>, fade_out: f32) -> Self {
        Self {
            name,
            parent,
            destroy_ui: None,
            components,
            fade_out,
        }
    }

    pub fn with_destroy_ui(mut self, destroy_ui: String) -> Self {
        self.destroy_ui = Some(destroy_ui);
        self
    }
}

impl ICuiElement for CuiElement {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_parent(&self) -> &str {
        &self.parent
    }

    fn get_fade_out(&self) -> f32 {
        self.fade_out
    }

    fn get_components(&self) -> &[ComponentType] {
        &self.components
    }

    fn get_destroy_ui(&self) -> Option<&str> {
        self.destroy_ui.as_deref()
    }
} 