use crate::{CuiElement, CuiPanel, CuiButton, CuiLabel};

#[derive(Clone, PartialEq)]
pub struct Element {
    pub id: String,
    pub element_type: ElementType,
    pub children: Vec<Element>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ElementType {
    Panel(CuiPanel),
    Button(CuiButton),
    Label(CuiLabel),
}

impl Element {
    pub fn from_cui_element(_element: &CuiElement) -> Self {
        // TODO: Implement conversion from CuiElement
        unimplemented!()
    }

    pub fn to_cui_element(&self) -> CuiElement {
        // TODO: Implement conversion to CuiElement
        unimplemented!()
    }
}

pub fn find_element_by_id<'a>(elements: &'a [Element], id: &str) -> Option<&'a Element> {
    for element in elements {
        if element.id == id {
            return Some(element);
        }
        if let Some(found) = find_element_by_id(&element.children, id) {
            return Some(found);
        }
    }
    None
}

pub fn find_element_by_id_mut<'a>(elements: &'a mut [Element], id: &str) -> Option<&'a mut Element> {
    for element in elements {
        if element.id == id {
            return Some(element);
        }
        if let Some(found) = find_element_by_id_mut(&mut element.children, id) {
            return Some(found);
        }
    }
    None
}

pub fn remove_element_by_id(elements: &mut Vec<Element>, id: &str) -> Option<Element> {
    let mut i = 0;
    while i < elements.len() {
        if elements[i].id == id {
            return Some(elements.remove(i));
        }
        if let Some(element) = remove_element_by_id(&mut elements[i].children, id) {
            return Some(element);
        }
        i += 1;
    }
    None
} 