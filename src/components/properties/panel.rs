use yew::prelude::*;
use crate::shared::lib::component::Component;
use crate::shared::ui::{Panel, Input};
use crate::shared::lib::types::Color;

#[derive(Properties, Clone, PartialEq)]
pub struct PropertiesPanelProps {
    pub component: Option<Component>,
    pub on_update_component: Callback<Component>,
}

#[function_component(PropertiesPanel)]
pub fn properties_panel(props: &PropertiesPanelProps) -> Html {
    let PropertiesPanelProps {
        component,
        on_update_component,
    } = props.clone();

    html! {
        <Panel 
            class="properties-panel"
            background_color={Some(Color::new(242, 242, 242, 255))}
        >
            if let Some(component) = component {
                <div class="component-properties">
                    <div class="property-group">
                        <h3>{"Component Type"}</h3>
                        <Input
                            value={component.component_type().to_string()}
                            onchange={Callback::noop()}
                            placeholder={"Component Type"}
                        />
                    </div>
                    // Здесь будут специфичные для компонента свойства
                </div>
            } else {
                <div class="no-selection">
                    {"No component selected"}
                </div>
            }
        </Panel>
    }
} 