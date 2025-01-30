use yew::prelude::*;
use web_sys::DragEvent;
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
    let element_class = if Some(props.element.id.clone()) == props.selected_id {
        "element-header selected"
    } else {
        "element-header"
    };

    let onclick = {
        let on_select = props.on_select.clone();
        let id = props.element.id.clone();
        Callback::from(move |_| {
            on_select.emit(id.clone());
        })
    };

    let ondragstart = {
        let id = props.element.id.clone();
        Callback::from(move |e: DragEvent| {
            if let Some(data_transfer) = e.data_transfer() {
                let _ = data_transfer.set_data("text/plain", &id);
            }
        })
    };

    let ondragover = Callback::from(|e: DragEvent| {
        e.prevent_default();
    });

    let ondrop = {
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
    };

    // Находим дочерние элементы
    let children: Vec<Element> = props.container.elements.iter()
        .filter(|e: &&CuiElement| e.parent == props.element.id)
        .map(|e| Element::from(e.clone()))
        .collect();

    html! {
        <div class="element-item">
            <div
                class={element_class}
                onclick={onclick}
                draggable="true"
                ondragstart={ondragstart}
                ondragover={ondragover}
                ondrop={ondrop}
            >
                <span class="element-name">{&props.element.id}</span>
                <span class="element-type">{"("}{format!("{:?}", &props.element.element_type)}{")"}</span>
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