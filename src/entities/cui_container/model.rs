use serde::{Serialize, Deserialize};
use crate::entities::cui_element::model::CuiElement;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuiContainer {
    pub elements: Vec<CuiElement>,
}

impl PartialEq for CuiContainer {
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
}

impl CuiContainer {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, mut element: CuiElement) -> String {
        let id = element.id.clone();
        if element.parent_id == Some("Overlay".to_string()) {
            self.elements.push(element);
        } else {
            self.add_element_to_parent(element);
        }
        id
    }

    fn add_element_to_parent(&mut self, mut element: CuiElement) {
        let parent_id = element.parent_id.clone();
        if let Some(parent_id) = parent_id {
            if let Some(parent) = self.find_element_mut(&parent_id) {
                parent.add_child(element);
            } else {
                element.parent_id = Some("Overlay".to_string());
                self.elements.push(element);
            }
        }
    }

    pub fn find_element(&self, id: &str) -> Option<&CuiElement> {
        self.find_element_recursive(id, &self.elements)
    }

    fn find_element_recursive<'a>(&'a self, id: &str, elements: &'a [CuiElement]) -> Option<&'a CuiElement> {
        for element in elements {
            if element.id == id {
                return Some(element);
            }
            if let Some(found) = self.find_element_recursive(id, &element.children) {
                return Some(found);
            }
        }
        None
    }

    pub fn find_element_mut(&mut self, id: &str) -> Option<&mut CuiElement> {
        for element in &mut self.elements {
            if element.id == id {
                return Some(element);
            }
            if let Some(found) = Self::find_element_mut_in_children(id, &mut element.children) {
                return Some(found);
            }
        }
        None
    }

    fn find_element_mut_in_children<'a>(id: &str, children: &'a mut [CuiElement]) -> Option<&'a mut CuiElement> {
        for child in children {
            if child.id == id {
                return Some(child);
            }
            if let Some(found) = Self::find_element_mut_in_children(id, &mut child.children) {
                return Some(found);
            }
        }
        None
    }

    pub fn remove_element(&mut self, id: &str) -> Option<CuiElement> {
        if let Some(index) = self.elements.iter().position(|e| e.id == id) {
            return Some(self.elements.remove(index));
        }

        for element in &mut self.elements {
            if let Some(removed) = Self::remove_element_from_children(id, &mut element.children) {
                return Some(removed);
            }
        }
        None
    }

    fn remove_element_from_children(id: &str, children: &mut Vec<CuiElement>) -> Option<CuiElement> {
        if let Some(index) = children.iter().position(|e| e.id == id) {
            return Some(children.remove(index));
        }

        for child in children {
            if let Some(removed) = Self::remove_element_from_children(id, &mut child.children) {
                return Some(removed);
            }
        }
        None
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    pub fn from_json(json: &str) -> Option<Self> {
        serde_json::from_str(json).ok()
    }

    pub fn generate_name(&self) -> String {
        Uuid::new_v4().to_string()
    }
} 