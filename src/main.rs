#![deny(clippy::disallowed_types)]

use yew::prelude::*;
use cui_builder::components::{
    canvas::InfiniteCanvas,
    toolbar::Toolbar,
    properties::panel::PropertiesPanel,
};
use cui_builder::models::Element;
use cui_builder::core::component::Component;
use cui_builder::components::toolbar::Tool;

#[function_component(App)]
pub fn app() -> Html {
    let elements = use_state(|| Vec::<Element>::new());
    let selected_id = use_state(|| None::<String>);
    let selected_tool = use_state(|| Tool::Select);

    let on_select = {
        let selected_id = selected_id.clone();
        Callback::from(move |id: String| {
            selected_id.set(Some(id));
        })
    };

    let on_reparent = {
        let elements = elements.clone();
        Callback::from(move |(child_id, new_parent_id): (String, String)| {
            let mut new_elements = (*elements).clone();
            
            // Находим элемент
            if let Some(element_idx) = new_elements.iter().position(|e| e.id == child_id) {
                // Обновляем parent
                let mut element = new_elements[element_idx].clone();
                element.parent = Some(new_parent_id);
                new_elements[element_idx] = element;
            }
            
            elements.set(new_elements);
        })
    };

    let on_update_component = {
        let elements = elements.clone();
        Callback::from(move |(element_id, component): (String, Component)| {
            let mut new_elements = (*elements).clone();
            
            // Находим элемент
            if let Some(element_idx) = new_elements.iter().position(|e| e.id == element_id) {
                let mut element = new_elements[element_idx].clone();
                
                // Обновляем компонент
                if let Some(idx) = element.components.iter().position(|c| match (c, &component) {
                    (Component::RectTransform(_), Component::RectTransform(_)) => true,
                    (Component::Button(_), Component::Button(_)) => true,
                    (Component::Text(_), Component::Text(_)) => true,
                    (Component::Image(_), Component::Image(_)) => true,
                    (Component::RawImage(_), Component::RawImage(_)) => true,
                    (Component::NeedsCursor(_), Component::NeedsCursor(_)) => true,
                    (Component::NeedsKeyboard(_), Component::NeedsKeyboard(_)) => true,
                    (Component::UnityCanvasTransform(_), Component::UnityCanvasTransform(_)) => true,
                    _ => false,
                }) {
                    element.components[idx] = component;
                } else {
                    element.components.push(component);
                }
                new_elements[element_idx] = element;
            }
            
            elements.set(new_elements);
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

    let on_tool_change = {
        let selected_tool = selected_tool.clone();
        Callback::from(move |tool: Tool| {
            selected_tool.set(tool);
        })
    };

    html! {
        <div class="app">
            <div class="sidebar">
                <Toolbar 
                    on_add_element={on_add_element.clone()}
                    scale={1.0}
                    on_zoom_in={Callback::from(|_| {})}
                    on_zoom_out={Callback::from(|_| {})}
                    on_zoom_reset={Callback::from(|_| {})}
                    on_tool_change={on_tool_change.clone()}
                    selected_tool={(*selected_tool).clone()}
                />
                <div class="element-list">
                    {for elements.iter().map(|element| {
                        html! {
                            <div 
                                key={element.id.clone()}
                                class={classes!(
                                    "element-item",
                                    selected_id.as_ref().map_or(false, |id| *id == element.id)
                                        .then_some("selected")
                                )}
                                onclick={
                                    let id = element.id.clone();
                                    let on_select = on_select.clone();
                                    Callback::from(move |_| on_select.emit(id.clone()))
                                }
                            >
                                <span class="element-type">{format!("{:?}", element.element_type)}</span>
                                <span class="element-name">{&element.name}</span>
                            </div>
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
                    on_add_element={on_add_element.clone()}
                />
            </div>
            <div class="properties">
                <PropertiesPanel
                    elements={(*elements).clone()}
                    selected_id={(*selected_id).clone()}
                    on_update_component={on_update_component.clone()}
                />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
