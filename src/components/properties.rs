use yew::prelude::*;
use crate::components::{Element, Component};

#[derive(Properties, PartialEq)]
pub struct PropertiesPanelProps {
    pub selected_element: Option<Element>,
    pub on_component_change: Callback<(String, Component)>,
}

#[function_component(PropertiesPanel)]
pub fn properties_panel(props: &PropertiesPanelProps) -> Html {
    if let Some(element) = &props.selected_element {
        html! {
            <div class="properties-panel">
                <h3>{"Свойства"}</h3>
                {for element.components.iter().map(|component| {
                    render_component(&element.id, component, &props.on_component_change)
                })}
            </div>
        }
    } else {
        html! {
            <div class="properties-panel">
                <h3>{"Свойства"}</h3>
                <p>{"Выберите элемент для редактирования свойств"}</p>
            </div>
        }
    }
}

fn render_component(_id: &str, component: &Component, _on_change: &Callback<(String, Component)>) -> Html {
    component.render_properties()
} 