use yew::prelude::*;
use crate::models::Element;
use crate::oxide_interface::CuiElementContainer;
use crate::components::properties::properties::Properties;

#[derive(Properties, PartialEq, Clone)]
pub struct PropertiesPanelProps {
    pub container: CuiElementContainer,
    pub selected_id: Option<String>,
}

#[function_component(PropertiesPanel)]
pub fn properties_panel(props: &PropertiesPanelProps) -> Html {
    let selected_element = props.selected_id.as_ref().and_then(|id| {
        props.container.elements.iter()
            .find(|e| e.name == *id)
            .map(|e| Element::from(e.clone()))
    });

    html! {
        <div class="properties-panel">
            if let Some(element) = selected_element {
                <Properties element={element} />
            } else {
                <div class="no-selection">
                    <p>{"Выберите элемент для просмотра его свойств"}</p>
                </div>
            }
        </div>
    }
} 