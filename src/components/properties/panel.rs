use yew::prelude::*;
use web_sys::console;
use crate::models::Element;

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

#[derive(Properties, PartialEq)]
pub struct PropertiesPanelProps {
    pub elements: Vec<Element>,
    pub selected_id: Option<String>,
}

#[function_component(PropertiesPanel)]
pub fn properties_panel(props: &PropertiesPanelProps) -> Html {
    let selected_element = props.selected_id.as_ref().and_then(|id| {
        console::log_1(&format!("Looking for element with id: {}", id).into());
        let element = find_element_by_id(&props.elements, id);
        if let Some(e) = element {
            console::log_1(&format!("Found element: {} with {} components", e.name, e.components.len()).into());
        } else {
            console::log_1(&"Element not found".into());
        }
        element
    });

    html! {
        <div class="properties-panel">
            {if let Some(element) = selected_element {
                html! {
                    <div class="properties-content">
                        <h3>{"Properties"}</h3>
                        <div class="property-group">
                            <h4>{"Element"}</h4>
                            <div class="property-row">
                                <label>{"Name"}</label>
                                <input type="text" value={element.name.clone()} />
                            </div>
                            <div class="property-row">
                                <label>{"Type"}</label>
                                <span>{element.element_type.to_string()}</span>
                            </div>
                        </div>
                        {for element.components.iter().map(|component| {
                            console::log_1(&format!("Rendering component: {}", component.component_type()).into());
                            component.render_properties()
                        })}
                    </div>
                }
            } else {
                html! {
                    <div class="no-selection">
                        <p>{"Select an element to view its properties"}</p>
                    </div>
                }
            }}
        </div>
    }
} 