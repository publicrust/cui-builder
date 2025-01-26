use yew::prelude::*;
use super::{Element, ElementType};
use super::component::RectTransformComponent;

#[derive(Properties, PartialEq)]
pub struct UnityCanvasProps {
    pub element: Element,
    pub on_element_move: Option<Callback<(String, RectTransformComponent)>>,
    pub on_select: Option<Callback<String>>,
}

#[function_component(UnityCanvas)]
pub fn unity_canvas(props: &UnityCanvasProps) -> Html {
    let element = &props.element;
    let transform = element.components.iter()
        .find_map(|c| c.as_any().downcast_ref::<RectTransformComponent>())
        .expect("UnityCanvas должен иметь RectTransformComponent");
    
    let style = format!(
        "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px;",
        transform.offset_min.0,
        transform.offset_min.1,
        transform.offset_max.0 - transform.offset_min.0,
        transform.offset_max.1 - transform.offset_min.1,
    );

    let on_mousedown = {
        let on_select = props.on_select.clone();
        let id = element.id.clone();
        Callback::from(move |_| {
            if let Some(on_select) = &on_select {
                on_select.emit(id.clone());
            }
        })
    };

    html! {
        <div class="unity-canvas" {style} onmousedown={on_mousedown}>
            <div class="unity-canvas-content">
                {for element.children.iter().map(|child| html! {
                    <UnityElement element={child.clone()} />
                })}
            </div>
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
        .find_map(|c| c.as_any().downcast_ref::<RectTransformComponent>())
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