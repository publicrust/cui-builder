use yew::prelude::*;
use crate::core::component::Component;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub component: Option<Component>,
    pub on_update_component: Callback<Component>,
}

#[function_component(PropertiesPanel)]
pub fn properties_panel(props: &Props) -> Html {
    let Props { component, on_update_component } = props;

    html! {
        <div class="properties-panel">
            <h2>{"Properties"}</h2>
            {
                if let Some(component) = component {
                    component.render_properties_with_callback(on_update_component.clone())
                } else {
                    html! {
                        <div class="no-selection">
                            {"No component selected"}
                        </div>
                    }
                }
            }
        </div>
    }
} 