mod components;

use components::{
    infinite_canvas::InfiniteCanvas,
    sidebar::Sidebar,
    properties::PropertiesPanel,
    Element, Transform, RectTransform,
};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let elements = use_state(Vec::<Element>::new);
    let selected_id = use_state(|| None::<String>);

    let on_add_element = {
        let elements = elements.clone();
        Callback::from(move |new_element: Element| {
            let mut new_elements = (*elements).clone();
            new_elements.push(new_element);
            elements.set(new_elements);
        })
    };

    let on_select = {
        let selected_id = selected_id.clone();
        Callback::from(move |id: String| {
            selected_id.set(Some(id));
        })
    };

    let on_transform_change = {
        let elements = elements.clone();
        Callback::from(move |(id, new_transform): (String, Transform)| {
            let mut new_elements = (*elements).clone();
            if let Some(element) = find_element_by_id_mut(&mut new_elements, &id) {
                element.transform = Some(new_transform);
                elements.set(new_elements);
            }
        })
    };

    let on_rect_transform_change = {
        let elements = elements.clone();
        Callback::from(move |(id, new_rect_transform): (String, RectTransform)| {
            let mut new_elements = (*elements).clone();
            if let Some(element) = find_element_by_id_mut(&mut new_elements, &id) {
                element.rect_transform = Some(new_rect_transform);
                elements.set(new_elements);
            }
        })
    };

    let on_reparent = {
        let elements = elements.clone();
        Callback::from(move |(element_id, new_parent_id): (String, Option<String>)| {
            let mut new_elements = (*elements).clone();
            if let Some(element) = remove_element_by_id(&mut new_elements, &element_id) {
                if let Some(parent_id) = new_parent_id {
                    if let Some(parent) = find_element_by_id_mut(&mut new_elements, &parent_id) {
                        parent.children.push(element);
                    }
                } else {
                    new_elements.push(element);
                }
                elements.set(new_elements);
            }
        })
    };

    let selected_element = {
        let elements = (*elements).clone();
        let selected_id = (*selected_id).clone();
        selected_id.and_then(|id| find_element_by_id(&elements, &id).cloned())
    };

    html! {
        <div class="app">
            <Sidebar
                elements={(*elements).clone()}
                on_select={on_select.clone()}
                on_add_element={on_add_element}
                on_reparent={on_reparent}
            />
            <div class="main-content">
                <InfiniteCanvas />
            </div>
            <PropertiesPanel
                selected_element={selected_element}
                on_transform_change={on_transform_change}
                on_rect_transform_change={on_rect_transform_change}
            />
        </div>
    }
}

fn find_element_by_id<'a>(elements: &'a [Element], id: &str) -> Option<&'a Element> {
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

fn find_element_by_id_mut<'a>(elements: &'a mut [Element], id: &str) -> Option<&'a mut Element> {
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

fn remove_element_by_id(elements: &mut Vec<Element>, id: &str) -> Option<Element> {
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

fn main() {
    yew::Renderer::<App>::new().render();
}
