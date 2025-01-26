mod components;

use components::{
    infinite_canvas::InfiniteCanvas,
    sidebar::Sidebar,
    properties::PropertiesPanel,
    Element, CuiComponent,
};
use yew::prelude::*;
use web_sys::console;

#[function_component(App)]
pub fn app() -> Html {
    let elements = use_state(|| vec![]);
    let selected_id = use_state(|| None::<String>);
    
    let on_select = {
        let selected_id = selected_id.clone();
        Callback::from(move |id: String| {
            console::log_1(&format!("MAIN: Выбран элемент {}", id).into());
            selected_id.set(Some(id));
        })
    };
    
    let on_add_element = {
        let elements = elements.clone();
        Callback::from(move |element: Element| {
            let mut new_elements = (*elements).clone();
            new_elements.push(element);
            elements.set(new_elements);
        })
    };
    
    let on_component_change = {
        let elements = elements.clone();
        Callback::from(move |(id, component): (String, Box<dyn CuiComponent>)| {
            let mut new_elements = (*elements).clone();
            if let Some(element) = new_elements.iter_mut().find(|e| e.id == id) {
                if let Some(existing_component) = element.components.iter_mut()
                    .find(|c| c.component_type() == component.component_type()) {
                    *existing_component = component;
                }
            }
            elements.set(new_elements);
        })
    };
    
    let on_reparent = {
        let elements = elements.clone();
        Callback::from(move |(id, parent_id): (String, Option<String>)| {
            let mut new_elements = (*elements).clone();
            
            // Находим и удаляем элемент из его текущего местоположения
            let element = remove_element_by_id(&mut new_elements, &id);
            
            if let Some(element) = element {
                match parent_id {
                    Some(parent_id) => {
                        // Добавляем элемент к новому родителю
                        if let Some(parent) = find_element_by_id_mut(&mut new_elements, &parent_id) {
                            parent.children.push(element);
                        }
                    }
                    None => {
                        // Добавляем элемент в корень
                        new_elements.push(element);
                    }
                }
                elements.set(new_elements);
            }
        })
    };
    
    let selected_element = {
        let elements = (*elements).clone();
        let selected_id = (*selected_id).clone();
        let element = selected_id.and_then(|id| {
            console::log_1(&format!("MAIN: Поиск элемента с id={}", id).into());
            let found = find_element_by_id(&elements, &id);
            if found.is_some() {
                console::log_1(&"MAIN: Элемент найден".into());
            } else {
                console::log_1(&"MAIN: Элемент не найден".into());
            }
            found.cloned()
        });
        element
    };

    html! {
        <div class="app">
            <Sidebar
                elements={(*elements).clone()}
                selected_id={(*selected_id).clone()}
                on_select={on_select.clone()}
                on_add_element={on_add_element}
                on_reparent={on_reparent.clone()}
            />
            
            <InfiniteCanvas
                elements={(*elements).clone()}
                selected_id={(*selected_id).clone()}
                on_select={on_select}
                on_reparent={on_reparent}
            />
            
            <PropertiesPanel
                selected_element={selected_element}
                on_component_change={on_component_change}
            />
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
