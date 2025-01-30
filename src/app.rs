use yew::prelude::*;
use crate::models::{Element, Component};
use crate::components::canvas::infinite::InfiniteCanvas;
use crate::components::sidebar::toolbar::Toolbar;
use crate::components::properties::panel::PropertiesPanel;
use crate::oxide_interface::CuiElementContainer;
use crate::oxide_interface::elements::cui_element::CuiElement;
use crate::oxide_interface::components::component_type::ComponentType;
use crate::oxide_interface::components::cui_rect_transform_component::CuiRectTransformComponent;

#[function_component(App)]
pub fn app() -> Html {
    let elements = use_state(Vec::new);
    let selected_id = use_state(|| None::<String>);

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

    html! {
        <div class="app">
            <div class="sidebar">
                <Toolbar on_add_element={on_add_element.clone()} />
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
                />
            </div>
        </div>
    }
} 