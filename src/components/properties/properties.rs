use yew::prelude::*;
use crate::models::Element;

#[derive(Properties, PartialEq)]
pub struct PropertiesProps {
    pub element: Element,
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
                    html! {
                        <div class="property-row">
                            <label>{component.component_type()}</label>
                        </div>
                    }
                })}
            </div>
        </div>
    }
} 