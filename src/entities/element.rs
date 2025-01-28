use serde::{Serialize, Deserialize};
use crate::shared::lib::component::Component;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Element {
    pub id: String,
    pub name: String,
    pub components: Vec<Component>,
    pub children: Vec<Element>,
}

impl Element {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            components: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn add_component(&mut self, component: Component) {
        self.components.push(component);
    }

    pub fn add_child(&mut self, child: Element) {
        self.children.push(child);
    }
} 