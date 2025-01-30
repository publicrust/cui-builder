use crate::models::element::Element;
use crate::oxide_interface::CuiElementContainer;
use crate::oxide_interface::elements::cui_element::CuiElement;

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

pub fn find_element_by_id<'a>(container: &'a CuiElementContainer, id: &str) -> Option<Element> {
    container.elements.iter()
        .find(|e| e.name == id)
        .map(|e| Element::from(e.clone()))
}

pub fn find_element_by_id_mut<'a>(container: &'a mut CuiElementContainer, id: &str) -> Option<&'a mut CuiElement> {
    container.elements.iter_mut()
        .find(|e| e.name == id)
}

pub fn remove_element_by_id(container: &mut CuiElementContainer, id: &str) -> Option<CuiElement> {
    if let Some(pos) = container.elements.iter().position(|e| e.name == id) {
        Some(container.elements.remove(pos))
    } else {
        None
    }
} 