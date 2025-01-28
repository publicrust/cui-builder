use yew::prelude::*;
use crate::entities::element::Element;
use crate::shared::lib::component::Component;

#[derive(Properties, Clone, PartialEq)]
pub struct UnityElementProps {
    pub element: Element,
    pub selected_id: Option<String>,
    pub on_select: Callback<String>,
    pub on_reparent: Callback<(String, Option<String>)>,
    pub on_update_component: Callback<(String, Component)>,
}

#[function_component(UnityElement)]
pub fn unity_element(props: &UnityElementProps) -> Html {
    let UnityElementProps {
        element,
        selected_id,
        on_select,
        on_reparent,
        on_update_component,
    } = props.clone();

    let is_selected = selected_id.as_ref().map_or(false, |id| id == &element.id);

    let onclick = {
        let id = element.id.clone();
        let on_select = on_select.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            on_select.emit(id.clone());
        })
    };

    let ondragstart = {
        let id = element.id.clone();
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
    };

    html! {
        <div
            class={classes!("unity-element", is_selected.then(|| "selected"))}
            draggable="true"
            {onclick}
            {ondragstart}
            {ondragover}
            {ondrop}
        >
            <div class="element-content">
                <span class="element-name">{&element.name}</span>
            </div>
            <div class="element-children">
                {for element.children.iter().map(|child| {
                    html! {
                        <UnityElement
                            element={child.clone()}
                            selected_id={selected_id.clone()}
                            on_select={on_select.clone()}
                            on_reparent={on_reparent.clone()}
                            on_update_component={on_update_component.clone()}
                        />
                    }
                })}
            </div>
        </div>
    }
} 