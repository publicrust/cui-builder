use yew::prelude::*;
use web_sys::{MouseEvent, console};
use crate::models::Element;
use crate::core::component::Component;

#[derive(Properties, PartialEq)]
pub struct UnityElementProps {
    pub element: Element,
    pub selected_id: Option<String>,
    pub on_select: Callback<String>,
    pub on_reparent: Callback<(String, Option<String>)>,
}

#[function_component(UnityElement)]
pub fn unity_element(props: &UnityElementProps) -> Html {
    console::log_1(&format!("Отрисовка элемента: {} (id: {})", props.element.name, props.element.id).into());
    
    let element_class = if Some(props.element.id.clone()) == props.selected_id {
        "unity-element selected"
    } else {
        "unity-element"
    };

    let onclick = {
        let on_select = props.on_select.clone();
        let id = props.element.id.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            console::log_1(&format!("Клик по элементу: {}", id).into());
            on_select.emit(id.clone());
        })
    };

    let style = if let Some(transform) = props.element.components.iter()
        .find(|c| c.component_type() == "UnityCanvasTransform")
        .and_then(|c| match c {
            Component::UnityCanvasTransform(t) => Some(t),
            _ => None,
        }) {
        console::log_1(&format!("Применяем UnityCanvasTransform для {}: x={}, y={}, width={}, height={}", 
            props.element.name, transform.x, transform.y, transform.width, transform.height).into());
        
        format!(
            "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px;",
            transform.x,
            transform.y,
            transform.width,
            transform.height
        )
    } else if let Some(transform) = props.element.components.iter()
        .find(|c| c.component_type() == "RectTransform")
        .and_then(|c| match c {
            Component::RectTransform(t) => Some(t),
            _ => None,
        }) {
        console::log_1(&format!("Применяем RectTransform для {}: anchor_min=({}, {}), anchor_max=({}, {}), offset_min=({}, {}), offset_max=({}, {})", 
            props.element.name,
            transform.anchor_min.0, transform.anchor_min.1,
            transform.anchor_max.0, transform.anchor_max.1,
            transform.offset_min.0, transform.offset_min.1,
            transform.offset_max.0, transform.offset_max.1).into());
        
        let left = format!("{}%", transform.anchor_min.0 * 100.0);
        let width = format!("{}%", (transform.anchor_max.0 - transform.anchor_min.0) * 100.0);
        
        let top = format!("{}%", (1.0 - transform.anchor_max.1) * 100.0);
        let height = format!("{}%", (transform.anchor_max.1 - transform.anchor_min.1) * 100.0);
        
        let left_offset = format!("{}px", transform.offset_min.0);
        let top_offset = format!("{}px", -transform.offset_max.1);
        let width_offset = format!("{}px", transform.offset_max.0 - transform.offset_min.0);
        let height_offset = format!("{}px", transform.offset_max.1 - transform.offset_min.1);

        let style = format!(
            "position: absolute; \
             left: calc({} + {}); \
             top: calc({} + {}); \
             width: calc({} + {}); \
             height: calc({} + {});",
            left, left_offset,
            top, top_offset,
            width, width_offset,
            height, height_offset
        );
        
        console::log_1(&format!("Сгенерированный стиль для {}: {}", props.element.name, style).into());
        style
    } else {
        console::log_1(&format!("Элемент {} не имеет компонента трансформации", props.element.name).into());
        String::new()
    };

    html! {
        <div
            class={element_class}
            data-type={props.element.element_type.to_string()}
            style={style}
            onclick={onclick}
        >
            {&props.element.name}
            {for props.element.children.iter().map(|child| {
                html! {
                    <UnityElement
                        element={child.clone()}
                        selected_id={props.selected_id.clone()}
                        on_select={props.on_select.clone()}
                        on_reparent={props.on_reparent.clone()}
                    />
                }
            })}
        </div>
    }
} 