use yew::prelude::*;
use crate::models::{Element, ElementType};
use crate::core::component::Component;

#[derive(Properties, PartialEq)]
pub struct PropertiesProps {
    pub element: Element,
    pub on_update_component: Callback<(String, Component)>,
}

#[function_component(Properties)]
pub fn properties(props: &PropertiesProps) -> Html {
    html! {
        <div class="properties-content">
            <h3>{"Свойства"}</h3>
            
            <div class="property-group">
                <h4>{"Элемент"}</h4>
                <div class="property-row">
                    <label>{"Имя"}</label>
                    <input type="text" value={props.element.name.clone()} />
                </div>
                <div class="property-row">
                    <label>{"Тип"}</label>
                    <span>{props.element.element_type.to_string()}</span>
                </div>
                <div class="property-row">
                    <label>{"Родитель"}</label>
                    <span>{props.element.parent.clone().unwrap_or_else(|| "Нет".to_string())}</span>
                </div>
            </div>

            <div class="property-group">
                <h4>{"Компоненты"}</h4>
                {for props.element.components.iter().map(|component| {
                    match component {
                        Component::UnityCanvasTransform(transform) => {
                            let on_update = {
                                let id = props.element.id.clone();
                                let on_update_component = props.on_update_component.clone();
                                Callback::from(move |component: Component| {
                                    on_update_component.emit((id.clone(), component));
                                })
                            };
                            transform.render_properties_with_callback(on_update)
                        },
                        _ => html! {
                            <div class="property-row">
                                <label>{component.component_type()}</label>
                            </div>
                        }
                    }
                })}
            </div>
        </div>
    }
} 