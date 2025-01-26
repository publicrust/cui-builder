use yew::prelude::*;
use crate::components::{Element, Component, ElementType};
use crate::components::component::RectTransformComponent;

#[derive(Properties, PartialEq)]
pub struct UnityCanvasProps {
    pub element: Element,
    pub selected_id: Option<String>,
    pub on_select: Callback<String>,
    pub on_reparent: Callback<(String, Option<String>)>,
}

fn get_rect_transform(element: &Element) -> Option<&RectTransformComponent> {
    element.components.iter()
        .find_map(|c| match c {
            Component::RectTransform(t) => Some(t),
            _ => None,
        })
}

#[function_component(UnityCanvas)]
pub fn unity_canvas(props: &UnityCanvasProps) -> Html {
    let element_class = if Some(props.element.id.clone()) == props.selected_id {
        "unity-canvas selected"
    } else {
        "unity-canvas"
    };

    let onclick = {
        let on_select = props.on_select.clone();
        let id = props.element.id.clone();
        Callback::from(move |_| {
            on_select.emit(id.clone());
        })
    };

    let style = if let Some(transform) = get_rect_transform(&props.element) {
        format!(
            "position: absolute; left: {}%; top: {}%; right: {}%; bottom: {}%;",
            transform.anchor_min.0 * 100.0,
            transform.anchor_min.1 * 100.0,
            (1.0 - transform.anchor_max.0) * 100.0,
            (1.0 - transform.anchor_max.1) * 100.0
        )
    } else {
        String::new()
    };

    html! {
        <div
            class={element_class}
            style={style}
            onclick={onclick}
        >
            {for props.element.children.iter().map(|child| {
                html! {
                    <UnityCanvas
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

#[derive(Properties, PartialEq)]
pub struct UnityElementProps {
    pub element: Element,
}

#[function_component(UnityElement)]
pub fn unity_element(props: &UnityElementProps) -> Html {
    let element = &props.element;
    let rect_transform = element.components.iter()
        .find_map(|c| match c {
            Component::RectTransform(t) => Some(t),
            _ => None,
        })
        .expect("Дочерние элементы должны иметь RectTransformComponent");

    let style = format!(
        "position: absolute; left: {}%; top: {}%; right: {}%; bottom: {}%;",
        rect_transform.anchor_min.0 * 100.0,
        rect_transform.anchor_min.1 * 100.0,
        (1.0 - rect_transform.anchor_max.0) * 100.0,
        (1.0 - rect_transform.anchor_max.1) * 100.0,
    );

    let element_class = match element.element_type {
        ElementType::UnityCanvas => "unity-canvas-element",
        ElementType::Panel => "panel-element",
        ElementType::Text => "text-element",
        ElementType::Button => "button-element",
    };

    html! {
        <div class={classes!("unity-element", element_class)} {style}>
            {for element.children.iter().map(|child| html! {
                <UnityElement element={child.clone()} />
            })}
        </div>
    }
} 