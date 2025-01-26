use crate::models::Element;

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