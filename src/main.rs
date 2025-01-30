#![deny(clippy::disallowed_types)]

use yew::prelude::*;
use cui_builder::components::{
    canvas::InfiniteCanvas,
    toolbar::Toolbar,
    properties::panel::PropertiesPanel,
};
use cui_builder::models::Element;
use cui_builder::oxide_interface::{
    CuiElementContainer,
    components::{
        component_type::ComponentType,
        ICuiComponent,
    },
    elements::cui_element::CuiElement,
};
use cui_builder::core::component::Component;
use cui_builder::components::toolbar::Tool;

#[function_component(App)]
pub fn app() -> Html {
    let container = use_state(|| CuiElementContainer::new());
    let selected_id = use_state(|| None::<String>);
    let selected_tool = use_state(|| Tool::Select);

    let on_select = {
        let selected_id = selected_id.clone();
        Callback::from(move |id: String| {
            selected_id.set(Some(id));
        })
    };

    let on_reparent = {
        let container = container.clone();
        Callback::from(move |(child_id, new_parent_id): (String, String)| {
            let mut new_container = (*container).clone();
            
            // Находим элемент
            if let Some(element_idx) = new_container.elements.iter().position(|e| e.name == child_id) {
                // Обновляем parent
                let mut element = new_container.elements[element_idx].clone();
                element.parent = new_parent_id;
                new_container.elements[element_idx] = element;
            }
            
            container.set(new_container);
        })
    };

    let on_update_component = {
        let container = container.clone();
        Callback::from(move |(element_id, component): (String, Component)| {
            let mut new_container = (*container).clone();
            
            // Находим элемент
            if let Some(element_idx) = new_container.elements.iter().position(|e| e.name == element_id) {
                // Конвертируем Component в ComponentType и обновляем
                let component_type = match component {
                    Component::RectTransform(t) => ComponentType::RectTransform(t),
                    Component::Button(b) => ComponentType::Button(b),
                    Component::Text(t) => ComponentType::Text(t),
                    Component::Image(i) => ComponentType::Image(i),
                    Component::RawImage(r) => ComponentType::RawImage(r),
                    Component::NeedsCursor(n) => ComponentType::NeedsCursor(n),
                    Component::NeedsKeyboard(n) => ComponentType::NeedsKeyboard(n),
                    Component::UnityCanvasTransform(_) => return,
                };
                
                let mut element = new_container.elements[element_idx].clone();
                if let Some(idx) = element.components.iter().position(|c| c.component_type() == component_type.component_type()) {
                    element.components[idx] = component_type;
                } else {
                    element.components.push(component_type);
                }
                new_container.elements[element_idx] = element;
            }
            
            container.set(new_container);
        })
    };

    let on_add_element = {
        let container = container.clone();
        Callback::from(move |element: Element| {
            let mut new_container = (*container).clone();
            let cui_element: CuiElement = element.into();
            new_container.add_element(cui_element);
            container.set(new_container);
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
                    {for container.elements.iter().map(|element| {
                        let element = Element::from(element.clone());
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
                    elements={(*container).elements.clone()}
                    selected_id={(*selected_id).clone()}
                    on_select={on_select.clone()}
                    on_reparent={on_reparent.clone()}
                    on_update_component={on_update_component.clone()}
                    on_add_element={on_add_element.clone()}
                />
            </div>
            <div class="properties">
                <PropertiesPanel
                    container={(*container).clone()}
                    selected_id={(*selected_id).clone()}
                />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
