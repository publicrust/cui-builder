mod shared;
mod entities;
mod components;
mod features;
mod widgets;

use yew::prelude::*;
use crate::entities::element::Element;
use crate::components::{
    canvas::infinite::InfiniteCanvas,
    sidebar::{element_item::ElementItem, toolbar::Toolbar},
    properties::panel::PropertiesPanel,
};
use crate::shared::lib::component::Component;

#[function_component(App)]
pub fn app() -> Html {
    let elements = use_state(Vec::new);
    let selected_id = use_state(|| None);
    let is_reparenting = use_state(|| false);
    let selected_component = use_state(|| None::<Component>);
    
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
            
            if let Some(child_element) = remove_element_by_id(&mut new_elements, &child_id) {
                match new_parent_id {
                    Some(parent_id) => {
                        if let Some(parent) = find_element_by_id_mut(&mut new_elements, &parent_id) {
                            parent.children.push(child_element);
                        } else {
                            new_elements.push(child_element);
                        }
                    }
                    None => {
                        new_elements.push(child_element);
                    }
                }
                elements.set(new_elements);
            }
            
            is_reparenting.set(false);
        })
    };
    
    let on_update_component = {
        let elements = elements.clone();
        let selected_id = selected_id.clone();
        let selected_component = selected_component.clone();
        Callback::from(move |(id, c): (String, Component)| {
            let mut new_elements = (*elements).clone();
            if let Some(element) = find_element_by_id_mut(&mut new_elements, &id) {
                if let Some(old) = element.components.iter_mut().find(|x| 
                    x.component_type() == c.component_type()
                ) {
                    *old = c.clone();
                }
            }
            elements.set(new_elements);
            selected_component.set(Some(c));
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
                    component={(*selected_component).clone()}
                    on_update_component={Callback::from(move |c: Component| {
                        if let Some(id) = (*selected_id).clone() {
                            on_update_component.emit((id, c));
                        }
                    })}
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
