use yew::prelude::*;
use web_sys::{DragEvent, MouseEvent};
use crate::models::element::Element;
use crate::oxide_interface::CuiElementContainer;
use crate::oxide_interface::elements::cui_element::CuiElement;

#[derive(Properties, PartialEq)]
pub struct ElementItemProps {
    pub element: Element,
    pub container: CuiElementContainer,
    pub selected_id: Option<String>,
    pub on_select: Callback<String>,
    pub on_reparent: Callback<(String, Option<String>)>,
}

#[function_component(ElementItem)]
pub fn element_item(props: &ElementItemProps) -> Html {
    let drag_over = use_state(|| false);

    let element_header_class = classes!(
        "element-header",
        (props.selected_id.as_ref().map_or(false, |id| id == &props.element.id)).then_some("selected"),
        (*drag_over).then_some("drag-over"),
    );

    let onclick = {
        let on_select = props.on_select.clone();
        let id = props.element.id.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            on_select.emit(id.clone());
        })
    };

    let ondragstart = {
        let id = props.element.id.clone();
        Callback::from(move |e: DragEvent| {
            if let Some(dt) = e.data_transfer() {
                let _ = dt.set_data("text/plain", &id);
            }
        })
    };

    let ondragover = {
        let drag_over = drag_over.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            drag_over.set(true);
        })
    };

    let ondragleave = {
        let drag_over = drag_over.clone();
        Callback::from(move |_: DragEvent| {
            drag_over.set(false);
        })
    };

    let ondrop = {
        let on_reparent = props.on_reparent.clone();
        let id = props.element.id.clone();
        let drag_over = drag_over.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            drag_over.set(false);
            if let Some(dt) = e.data_transfer() {
                if let Ok(child_id) = dt.get_data("text/plain") {
                    if child_id != id {
                        on_reparent.emit((child_id, Some(id.clone())));
                    }
                }
            }
        })
    };

    // Находим дочерние элементы
    let children = {
        let elements = &props.container.elements;
        let parent_id: &String = &props.element.id;
        
        elements
            .iter()
            .filter(|e: &&CuiElement| {
                let parent_opt = &e.parent;
                parent_opt
                    .as_ref()
                    .map_or(false, |p: &String| p == parent_id)
            })
            .cloned()
            .map(Element::from)
            .collect::<Vec<Element>>()
    };

    html! {
        <div class="element-item">
            <div
                class={element_header_class}
                onclick={onclick}
                draggable="true"
                ondragstart={ondragstart}
                ondragover={ondragover}
                ondragleave={ondragleave}
                ondrop={ondrop}
            >
                <span class="element-type">{format!("{:?}", &props.element.element_type)}</span>
                <span class="element-name">{&props.element.id}</span>
            </div>
            if !children.is_empty() {
                <div class="element-children">
                    {for children.iter().map(|child| {
                        html! {
                            <ElementItem
                                element={child.clone()}
                                container={props.container.clone()}
                                selected_id={props.selected_id.clone()}
                                on_select={props.on_select.clone()}
                                on_reparent={props.on_reparent.clone()}
                            />
                        }
                    })}
                </div>
            }
        </div>
    }
} 