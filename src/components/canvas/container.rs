use yew::prelude::*;
use crate::entities::cui_container::model::CuiContainer;
use crate::entities::cui_element::model::CuiElement;
use crate::entities::cui_element::components::rect_transform::RectTransform;
use web_sys::DragEvent;

#[derive(Properties, PartialEq)]
pub struct CuiCanvasProps {
    pub container: CuiContainer,
    pub selected_id: Option<String>,
    pub on_select: Callback<String>,
    pub on_reparent: Callback<(String, Option<String>)>,
}

fn render_element(
    element: &CuiElement,
    selected_id: &Option<String>,
    on_select: &Callback<String>,
    on_reparent: &Callback<(String, Option<String>)>,
) -> Html {
    let element_class = if selected_id.as_ref().map(|id| id == &element.id).unwrap_or(false) {
        "cui-element selected"
    } else {
        "cui-element"
    };

    let onclick = {
        let on_select = on_select.clone();
        let id = element.id.clone();
        Callback::from(move |_| {
            on_select.emit(id.clone());
        })
    };

    let style = if let Some(transform) = element.get_component::<RectTransform>() {
        format!(
            "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px; transform: rotate({}deg) scale({}, {});",
            transform.position.x,
            transform.position.y,
            transform.size.width,
            transform.size.height,
            transform.rotation,
            transform.scale.x,
            transform.scale.y
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
                let on_reparent = on_reparent.clone();
                let id = element.id.clone();
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
            {for element.children.iter().map(|child| render_element(child, selected_id, on_select, on_reparent))}
        </div>
    }
}

#[function_component(CuiCanvas)]
pub fn cui_canvas(props: &CuiCanvasProps) -> Html {
    let CuiCanvasProps {
        container,
        selected_id,
        on_select,
        on_reparent,
    } = props.clone();

    html! {
        <div class="cui-canvas">
            {for container.elements.iter().map(|element| render_element(element, &selected_id, &on_select, &on_reparent))}
        </div>
    }
} 