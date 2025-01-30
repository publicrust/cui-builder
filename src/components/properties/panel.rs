use yew::prelude::*;
use crate::models::Element;
use crate::core::component::Component;
use crate::components::properties::properties::Properties;

#[derive(Properties, PartialEq, Clone)]
pub struct PropertiesPanelProps {
    pub elements: Vec<Element>,
    pub selected_id: Option<String>,
    pub on_update_component: Callback<(String, Component)>,
}

#[function_component(PropertiesPanel)]
pub fn properties_panel(props: &PropertiesPanelProps) -> Html {
    let selected_element = props.selected_id.as_ref().and_then(|id| {
        props.elements.iter()
            .find(|e| e.id == *id)
            .cloned()
    });

    html! {
        <div class="properties-panel">
            if let Some(element) = selected_element {
                <Properties 
                    element={element} 
                    on_update_component={props.on_update_component.clone()}
                />
            } else {
                <div class="no-selection">
                    <p>{"Выберите элемент для просмотра его свойств"}</p>
                </div>
            }
        </div>
    }
} 