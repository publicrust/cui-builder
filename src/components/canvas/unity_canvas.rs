use yew::prelude::*;
use web_sys::DragEvent;
use crate::core::{
    element::Element,
    component::{Component, unity_transform::UnityTransform},
};

#[derive(Properties, Clone, PartialEq)]
pub struct UnityCanvasProps {
    pub id: String,
    pub element: Element,
    pub on_select: Callback<String>,
    pub on_reparent: Callback<(String, String)>,
}

#[function_component(UnityCanvas)]
pub fn unity_canvas(props: &UnityCanvasProps) -> Html {
    let transform = props.element.components.iter()
        .find_map(|c| {
            if let Component::UnityTransform(t) = c {
                Some(t)
            } else {
                None
            }
        })
        .unwrap_or(&UnityTransform::default());

    let style = format!(
        "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px; background: white; border: 1px solid black;",
        transform.x, transform.y, transform.width, transform.height
    );

    let onclick = {
        let id = props.id.clone();
        let on_select = props.on_select.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            on_select.emit(id.clone());
        })
    };

    let ondragover = Callback::from(|e: DragEvent| {
        e.prevent_default();
    });

    let ondrop = {
        let id = props.id.clone();
        let on_reparent = props.on_reparent.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            if let Some(data) = e.data_transfer() {
                if let Ok(element_id) = data.get_data("text/plain") {
                    on_reparent.emit((element_id, id.clone()));
                }
            }
        })
    };

    html! {
        <div 
            class="unity-canvas"
            {style}
            {onclick}
            {ondragover}
            {ondrop}
        >
            {props.element.name.clone()}
        </div>
    }
} 