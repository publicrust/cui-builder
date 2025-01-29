#![deny(clippy::disallowed_types)]

use yew::prelude::*;
use cui_builder::{
    core::component::Component,
    components::{
        canvas::infinite::InfiniteCanvas,
        sidebar::{
            toolbar::Toolbar,
            element_item::ElementItem,
        },
        properties::panel::PropertiesPanel,
    },
    models::element::Element,
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

            selected_id.set(Some(id));
        })
    };
    
    let on_reparent = {
        let elements = elements.clone();
        let is_reparenting = is_reparenting.clone();
        Callback::from(move |(child_id, new_parent_id): (String, Option<String>)| {
            if *is_reparenting {

                return;
            }
            is_reparenting.set(true);
            

            
            let mut new_elements = (*elements).clone();
            
            // Находим и удаляем элемент из любого места в иерархии
            if let Some(child_element) = remove_element_by_id(&mut new_elements, &child_id) {

                
                match new_parent_id {
                    // Добавляем элемент к новому родителю
                    Some(parent_id) => {

                        if let Some(parent) = find_element_by_id_mut(&mut new_elements, &parent_id) {

                            parent.children.push(child_element);

                        } else {

                            new_elements.push(child_element);
                        }
                    }
                    // Добавляем элемент в корень
                    None => {

                        new_elements.push(child_element);
                    }
                }
                elements.set(new_elements);
            } else {

            }
            
            is_reparenting.set(false);
        })
    };
    
    let on_update_component = {
        let elements = elements.clone();
        Callback::from(move |(element_id, new_component): (String, Component)| {
            console::log_1(&format!("Начало обновления компонента для элемента: {} (тип: {})",
                element_id, new_component.component_type()).into());
            
            let mut new_elements = (*elements).clone();
            
            if let Some(element) = find_element_by_id_mut(&mut new_elements, &element_id) {
                console::log_1(&format!("Найден элемент для обновления: {} ({})", element.name, element.id).into());
                
                let mut updated = false;
                // Находим компонент того же типа и обновляем его
                for component in element.components.iter_mut() {
                    if component.component_type() == new_component.component_type() {
                        *component = new_component.clone();
                        updated = true;
                        console::log_1(&format!("Компонент {} успешно обновлен", component.component_type()).into());
                        break;
                    }
                }
                
                if updated {
                    elements.set(new_elements);
                    console::log_1(&"Состояние элементов успешно обновлено".into());
                } else {
                    console::log_1(&format!("Компонент типа {} не найден в элементе", 
                        new_component.component_type()).into());
                }
            } else {
                console::log_1(&format!("Элемент с id {} не найден для обновления компонента", 
                    element_id).into());
            }
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
                    on_update_component={on_update_component.clone()}
                />
            </div>
            <div class="properties-panel">
                <PropertiesPanel
                    elements={(*elements).clone()}
                    selected_id={(*selected_id).clone()}
                    on_update_component={on_update_component.clone()}
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

        if element.id == id {

            return Some(element);
        }
        if !element.children.is_empty() {

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

        if elements[i].id == id {

            return Some(elements.remove(i));
        }
        if !elements[i].children.is_empty() {

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
