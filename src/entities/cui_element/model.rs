use serde::{Serialize, Deserialize};
use super::components::interface::CuiComponent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuiElement {
    pub id: String,
    pub parent_id: Option<String>,
    #[serde(bound = "")]
    pub components: Vec<Box<dyn CuiComponent>>,
    pub children: Vec<CuiElement>,
}

impl CuiElement {
    pub fn new(id: String, parent_id: Option<String>) -> Self {
        Self {
            id,
            parent_id,
            components: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn add_component(&mut self, component: Box<dyn CuiComponent>) {
        self.components.push(component);
    }

    pub fn add_child(&mut self, child: CuiElement) {
        self.children.push(child);
    }

    pub fn get_component<T: CuiComponent + 'static>(&self) -> Option<&T> {
        self.components.iter().find_map(|c| {
            (&**c).downcast_ref()
        })
    }

    pub fn get_component_mut<T: CuiComponent + 'static>(&mut self) -> Option<&mut T> {
        self.components.iter_mut().find_map(|c| (&mut **c).downcast_mut())
    }
}

impl PartialEq for CuiElement {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && 
        self.parent_id == other.parent_id && 
        self.children.len() == other.children.len() &&
        self.children.iter().zip(other.children.iter()).all(|(a, b)| a == b)
        // Не сравниваем components, так как они содержат trait objects
    }
}

// Удаляем все реализации ComponentType и From преобразования 