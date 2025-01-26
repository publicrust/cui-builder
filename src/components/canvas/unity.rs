use yew::prelude::*;
use crate::models::Element;
use crate::core::component::Component;
use web_sys::DragEvent;

#[derive(Properties, PartialEq)]
pub struct UnityCanvasProps {
    pub element: Element,
    pub selected_id: Option<String>,
    pub on_select: Callback<String>,
    pub on_reparent: Callback<(String, Option<String>)>,
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

    let style = if let Some(transform) = props.element.components.iter()
        .find(|c| c.component_type() == "RectTransform")
        .and_then(|c| match c {
            Component::RectTransform(t) => Some(t),
            _ => None,
        }) {
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
            ondragover={Callback::from(|e: DragEvent| e.prevent_default())}
            ondrop={{
                let on_reparent = props.on_reparent.clone();
                let id = props.element.id.clone();
                Callback::from(move |e: DragEvent| {
                    e.prevent_default();
                    if let Some(data_transfer) = e.data_transfer() {
                        if let Ok(child_id) = data_transfer.get_data("text/plain") {
                            if child_id != id {
                                on_reparent.emit((child_id, Some(id.clone())));
                            }
                        }
                    }
                })
            }}
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