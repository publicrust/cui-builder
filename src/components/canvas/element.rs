use yew::prelude::*;
use web_sys::DragEvent;
use web_sys::MouseEvent;
use crate::models::Element;
use crate::core::component::Component;
use crate::oxide_interface::{
    components::{
        cui_rect_transform_component::CuiRectTransformComponent,
        component_type::ComponentType,
        ICuiComponent,
    },
    elements::cui_element::CuiElement,
};

#[derive(Properties, Clone, PartialEq)]
pub struct ElementProps {
    pub element: CuiElement,
    pub selected_id: Option<String>,
    pub on_select: Callback<String>,
    pub on_reparent: Callback<(String, String)>,
    pub on_update_component: Callback<(String, Component)>,
}

#[function_component(UnityElement)]
pub fn unity_element(props: &ElementProps) -> Html {
    let transform = props.element.components.iter()
        .find_map(|c| match c {
            ComponentType::RectTransform(t) => Some(t),
            _ => None,
        })
        .cloned()
        .unwrap_or_default();

    let element_class = if Some(props.element.name.clone()) == props.selected_id {
        "unity-element selected"
    } else {
        "unity-element"
    };

    let style = format!(
        "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px;",
        transform.base.anchormin, transform.base.anchormax, transform.base.offsetmin, transform.base.offsetmax
    );

    let onclick = {
        let id = props.element.name.clone();
        let on_select = props.on_select.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            on_select.emit(id.clone());
        })
    };

    let ondragstart = {
        let id = props.element.name.clone();
        Callback::from(move |e: DragEvent| {
            if let Some(dt) = e.data_transfer() {
                let _ = dt.set_data("text/plain", &id);
            }
        })
    };

    html! {
        <div 
            class={element_class}
            {style}
            {onclick}
            draggable="true"
            {ondragstart}
        >
            {props.element.name.clone()}
        </div>
    }
} 