use yew::prelude::*;
use crate::models::{Element, Component};
use crate::components::canvas::infinite::InfiniteCanvas;
use crate::components::sidebar::toolbar::Toolbar;
use crate::components::properties::panel::PropertiesPanel;

#[function_component(App)]
pub fn app() -> Html {
    let elements = use_state(|| vec![
        Element {
            id: "1".to_string(),
            name: "Root".to_string(),
            element_type: "Panel".to_string(),
            components: vec![
                Component::UnityCanvasTransform {
                    x: 100.0,
                    y: 100.0,
                    width: 200.0,
                    height: 200.0,
                }
            ],
            children: vec![],
        }
    ]);
    
    let selected_id = use_state(|| None::<String>);

    let on_select = {
        let selected_id = selected_id.clone();
        Callback::from(move |id: String| {
            selected_id.set(Some(id));
        })
    };

    let on_reparent = {
        let elements = elements.clone();
        Callback::from(move |(child_id, new_parent_id): (String, Option<String>)| {
            let mut new_elements = (*elements).clone();
            
            // Находим и удаляем элемент из старого родителя
            let mut child_element = None;
            let mut stack = vec![];
            for element in new_elements.iter_mut() {
                stack.push(element);
            }
            
            while let Some(element) = stack.pop() {
                if let Some(idx) = element.children.iter().position(|e| e.id == child_id) {
                    child_element = Some(element.children.remove(idx));
                    break;
                }
                for child in element.children.iter_mut() {
                    stack.push(child);
                }
            }

            // Добавляем элемент к новому родителю
            if let Some(child) = child_element {
                if let Some(parent_id) = new_parent_id {
                    let mut stack = vec![];
                    for element in new_elements.iter_mut() {
                        stack.push(element);
                    }
                    
                    while let Some(element) = stack.pop() {
                        if element.id == parent_id {
                            element.children.push(child);
                            break;
                        }
                        for child in element.children.iter_mut() {
                            stack.push(child);
                        }
                    }
                } else {
                    new_elements.push(child);
                }
            }
            
            elements.set(new_elements);
        })
    };

    let on_element_update = {
        let elements = elements.clone();
        Callback::from(move |updated_element: Element| {
            let mut new_elements = (*elements).clone();
            
            // Обновляем элемент в дереве
            let mut stack = vec![];
            for element in new_elements.iter_mut() {
                stack.push(element);
            }
            
            while let Some(element) = stack.pop() {
                if element.id == updated_element.id {
                    *element = updated_element.clone();
                    break;
                }
                for child in element.children.iter_mut() {
                    stack.push(child);
                }
            }
            
            elements.set(new_elements);
        })
    };

    html! {
        <div class="app">
            <div class="sidebar">
                <Toolbar />
            </div>
            <div class="main-content">
                <InfiniteCanvas
                    elements={(*elements).clone()}
                    selected_id={(*selected_id).clone()}
                    on_select={on_select.clone()}
                    on_reparent={on_reparent.clone()}
                    on_element_update={on_element_update.clone()}
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