use yew::prelude::*;
use web_sys::{MouseEvent, DragEvent, console};
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
            console::log_1(&format!("Clicked element: {}", id).into());
            on_select.emit(id.clone());
        })
    };

    let ondragstart = {
        let id = props.element.id.clone();
        Callback::from(move |e: DragEvent| {
            console::log_1(&format!("Started dragging element: {}", id).into());
            if let Some(data_transfer) = e.data_transfer() {
                let _ = data_transfer.set_data("text/plain", &id);
                console::log_1(&"Set drag data".into());
            }
        })
    };

    let ondragover = Callback::from(|e: DragEvent| {
        e.prevent_default();
        console::log_1(&"Dragging over element".into());
    });

    let ondrop = {
        let on_reparent = props.on_reparent.clone();
        let id = props.element.id.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();
            console::log_1(&format!("Dropping onto element: {}", id).into());
            if let Some(data_transfer) = e.data_transfer() {
                if let Ok(child_id) = data_transfer.get_data("text/plain") {
                    console::log_1(&format!("Got drag data: {}", child_id).into());
                    if child_id != id {
                        console::log_1(&format!("Reparenting {} to {}", child_id, id).into());
                        on_reparent.emit((child_id, Some(id.clone())));
                    }
                }
            }
        })
    };

    let style = if let Some(transform) = props.element.components.iter()
        .find(|c| c.component_type() == "UnityCanvasTransform")
        .and_then(|c| match c {
            Component::UnityCanvasTransform(t) => Some(t),
            _ => None,
        }) {
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
        format!(
            "position: absolute; left: {}%; top: {}%; width: {}%; height: {}%;",
            transform.offset_min.0 * 100.0,
            transform.offset_min.1 * 100.0,
            (transform.offset_max.0 - transform.offset_min.0) * 100.0,
            (transform.offset_max.1 - transform.offset_min.1) * 100.0
        )
    } else {
        String::new()
    };

    html! {
        <div
            class={element_class}
            data-type={props.element.element_type.to_string()}
            style={style}
            onclick={onclick}
            draggable="true"
            ondragstart={ondragstart}
            ondragover={ondragover}
            ondrop={ondrop}
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