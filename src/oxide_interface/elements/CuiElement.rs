use serde::Serialize;
use crate::oxide_interface::components::ICuiComponent;

#[derive(Serialize)]
pub struct CuiElement {
    pub name: String,
    pub parent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroy_ui: Option<String>,
    pub components: Vec<Box<dyn ICuiComponent>>,
    pub fade_out: f32,
}

impl CuiElement {
    pub fn new(name: String, parent: String, components: Vec<Box<dyn ICuiComponent>>, fade_out: f32) -> Self {
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