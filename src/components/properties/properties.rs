use yew::prelude::*;
use crate::models::Element;
use crate::core::component::Component;
use crate::core::component::properties::RenderProperties;

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
                    let on_update = {
                        let id = props.element.id.clone();
                        let on_update_component = props.on_update_component.clone();
                        Callback::from(move |component: Component| {
                            on_update_component.emit((id.clone(), component));
                        })
                    };

                    match component {
                        Component::RectTransform(transform) => transform.render_properties_with_callback(on_update),
                        Component::Button(button) => button.render_properties_with_callback(on_update),
                        Component::Text(text) => text.render_properties_with_callback(on_update),
                        Component::Image(image) => image.render_properties_with_callback(on_update),
                        Component::RawImage(raw_image) => raw_image.render_properties_with_callback(on_update),
                        Component::NeedsCursor(cursor) => cursor.render_properties_with_callback(on_update),
                        Component::NeedsKeyboard(keyboard) => keyboard.render_properties_with_callback(on_update),
                        Component::UnityCanvasTransform(transform) => transform.render_properties_with_callback(on_update),
                    }
                })}
            </div>
        </div>
    }
} 