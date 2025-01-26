#![deny(clippy::disallowed_types)]

use yew::prelude::*;
use cui_builder::{
    Element,
    components::{
        canvas::infinite::InfiniteCanvas,
        sidebar::{element_item::ElementItem, toolbar::Toolbar},
        properties::panel::PropertiesPanel,
    },
};
use web_sys::console;

#[function_component(App)]
pub fn app() -> Html {
    let elements = use_state(Vec::new);
    let selected_id = use_state(|| None);
    let is_reparenting = use_state(|| false);
    
    let on_add_element = {
        let elements = elements.clone();
        Callback::from(move |element: Element| {
            let mut new_elements = (*elements).clone();
            new_elements.push(element);
            elements.set(new_elements);
        })
    };
    
    let on_select = {
        let selected_id = selected_id.clone();
        Callback::from(move |id: String| {
            console::log_1(&format!("Selected element: {}", id).into());
            selected_id.set(Some(id));
        })
    };
    
    let on_reparent = {
        let elements = elements.clone();
        let is_reparenting = is_reparenting.clone();
        Callback::from(move |(child_id, new_parent_id): (String, Option<String>)| {
            if *is_reparenting {
                console::log_1(&"Already reparenting, skipping".into());
                return;
            }
            is_reparenting.set(true);
            
            console::log_1(&format!("Reparenting {} to {:?}", child_id, new_parent_id).into());
            
            let mut new_elements = (*elements).clone();
            
            // Находим и удаляем элемент из любого места в иерархии
            if let Some(child_element) = remove_element_by_id(&mut new_elements, &child_id) {
                console::log_1(&format!("Found and removed child element: {} ({})", child_element.name, child_element.id).into());
                
                match new_parent_id {
                    // Добавляем элемент к новому родителю
                    Some(parent_id) => {
                        console::log_1(&format!("Looking for parent: {}", parent_id).into());
                        if let Some(parent) = find_element_by_id_mut(&mut new_elements, &parent_id) {
                            console::log_1(&format!("Found parent: {} ({})", parent.name, parent.id).into());
                            parent.children.push(child_element);
                            console::log_1(&"Added child to new parent".into());
                        } else {
                            console::log_1(&"Parent not found, adding back to root".into());
                            new_elements.push(child_element);
                        }
                    }
                    // Добавляем элемент в корень
                    None => {
                        console::log_1(&"Adding child to root".into());
                        new_elements.push(child_element);
                    }
                }
                elements.set(new_elements);
            } else {
                console::log_1(&"Child element not found".into());
            }
            
            is_reparenting.set(false);
        })
    };
    
    html! {
        <div class="app">
            <div class="sidebar">
                <div class="toolbar">
                    <Toolbar {on_add_element} />
                </div>
                <div class="hierarchy">
                    {for (*elements).iter().map(|element| {
                        html! {
                            <ElementItem
                                element={element.clone()}
                                selected_id={(*selected_id).clone()}
                                on_select={on_select.clone()}
                                on_reparent={on_reparent.clone()}
                            />
                        }
                    })}
                </div>
            </div>
            <div class="workspace">
                <InfiniteCanvas
                    elements={(*elements).clone()}
                    selected_id={(*selected_id).clone()}
                    on_select={on_select.clone()}
                    on_reparent={on_reparent.clone()}
                />
            </div>
            <div class="properties-panel">
                <PropertiesPanel
                    elements={(*elements).clone()}
                    selected_id={(*selected_id).clone()}
                />
            </div>
        </div>
    }
}

fn find_element_by_id<'a>(elements: &'a [Element], id: &str) -> Option<&'a Element> {
    for element in elements {
        if element.id == id {
            return Some(element);
        }
        // Ищем в дочерних элементах Canvas
        if let Some(found) = find_element_by_id(&element.children, id) {
            return Some(found);
        }
    }
    None
}

fn find_element_by_id_mut<'a>(elements: &'a mut [Element], id: &str) -> Option<&'a mut Element> {
    for element in elements {
        console::log_1(&format!("Checking element: {} ({})", element.name, element.id).into());
        if element.id == id {
            console::log_1(&format!("Found element: {} ({})", element.name, element.id).into());
            return Some(element);
        }
        if !element.children.is_empty() {
            console::log_1(&format!("Checking children of: {} ({})", element.name, element.id).into());
            if let Some(found) = find_element_by_id_mut(&mut element.children, id) {
                return Some(found);
            }
        }
    }
    None
}

fn remove_element_by_id(elements: &mut Vec<Element>, id: &str) -> Option<Element> {
    let mut i = 0;
    while i < elements.len() {
        console::log_1(&format!("Checking for removal: {} ({})", elements[i].name, elements[i].id).into());
        if elements[i].id == id {
            console::log_1(&format!("Removing element: {} ({})", elements[i].name, elements[i].id).into());
            return Some(elements.remove(i));
        }
        if !elements[i].children.is_empty() {
            console::log_1(&format!("Checking children of: {} ({})", elements[i].name, elements[i].id).into());
            if let Some(element) = remove_element_by_id(&mut elements[i].children, id) {
                return Some(element);
            }
        }
        i += 1;
    }
    None
}

fn main() {
    yew::Renderer::<App>::new().render();
}
