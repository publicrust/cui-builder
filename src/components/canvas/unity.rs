use yew::prelude::*;
use web_sys::DragEvent;
use uuid::Uuid;
use crate::models::element::{Element, ElementType, UnityCanvasElement};
use crate::core::component::{Component, unity_canvas::UnityCanvasTransform};
use crate::oxide_interface::elements::cui_element::CuiElement;
use super::element::{UnityElement, ElementProps};

#[derive(Properties, Clone, PartialEq)]
pub struct UnityCanvasProps {
    pub canvas: UnityCanvasElement,
    pub selected_id: Option<String>,
    pub on_select: Callback<String>,
    pub on_reparent: Callback<(String, String)>,
    pub on_update_component: Callback<(String, Component)>,
    pub on_add_element: Callback<Element>,
}

#[function_component(UnityCanvas)]
pub fn unity_canvas(props: &UnityCanvasProps) -> Html {
    let ondragover = Callback::from(|e: DragEvent| {
        e.prevent_default();
    });

    let ondrop = {
        let on_add_element = props.on_add_element.clone();
        let canvas_id = props.canvas.id.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            if let Some(dt) = e.data_transfer() {
                if let Ok(element_id) = dt.get_data("text/plain") {
                    on_add_element.emit(Element {
                        id: Uuid::new_v4().to_string(),
                        name: element_id.clone(),
                        parent: Some(canvas_id.clone()),
                        element_type: ElementType::Panel,
                        components: vec![],
                        fade_out: 0.0,
                        destroy_ui: None,
                    });
                }
            }
        })
    };

    let style = format!(
        "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px;",
        props.canvas.transform.x,
        props.canvas.transform.y,
        props.canvas.transform.width,
        props.canvas.transform.height
    );

    html! {
        <div 
            class="unity-canvas"
            {style}
            {ondragover}
            {ondrop}
        >
            {for props.canvas.elements.iter().map(|element| {
                let element_props = ElementProps {
                    element: element.clone(),
                    selected_id: props.selected_id.clone(),
                    on_select: props.on_select.clone(),
                    on_reparent: props.on_reparent.clone(),
                    on_update_component: props.on_update_component.clone(),
                };

                html! {
                    <UnityElement ..element_props />
                }
            })}
        </div>
    }
} 