use yew::prelude::*;
use crate::components::{Element, CuiComponent};

#[derive(Properties, PartialEq)]
pub struct PropertiesPanelProps {
    pub selected_element: Option<Element>,
    pub on_component_change: Callback<(String, Box<dyn CuiComponent>)>,
}

#[function_component(PropertiesPanel)]
pub fn properties_panel(props: &PropertiesPanelProps) -> Html {
    if let Some(element) = &props.selected_element {
        let id = element.id.clone();
        
        html! {
            <div class="properties-panel">
                <h3>{"Свойства элемента"}</h3>
                {for element.components.iter().map(|component| {
                    render_component(&id, component, &props.on_component_change)
                })}
            </div>
        }
    } else {
        html! {
            <div class="properties-panel empty">
                {"Выберите элемент для редактирования"}
            </div>
        }
    }
}

fn render_component(_id: &str, component: &Box<dyn CuiComponent>, _on_change: &Callback<(String, Box<dyn CuiComponent>)>) -> Html {
    component.render_properties()
} 