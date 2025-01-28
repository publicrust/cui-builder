use yew::prelude::*;
use crate::entities::element::Element;

#[derive(Properties, Clone, PartialEq)]
pub struct ElementItemProps {
    pub element: Element,
    pub selected_id: Option<String>,
    pub on_select: Callback<String>,
    pub on_reparent: Callback<(String, Option<String>)>,
}

#[function_component(ElementItem)]
pub fn element_item(props: &ElementItemProps) -> Html {
    let ElementItemProps {
        element,
        selected_id,
        on_select,
        on_reparent,
    } = props.clone();

    let is_selected = selected_id.as_ref().map_or(false, |id| id == &element.id);
    let is_expanded = use_state(|| false);

    let onclick = {
        let id = element.id.clone();
        let on_select = on_select.clone();
        Callback::from(move |_| {
            on_select.emit(id.clone());
        })
    };

    let toggle_expanded = {
        let is_expanded = is_expanded.clone();
        Callback::from(move |_| {
            is_expanded.set(!*is_expanded);
        })
    };

    html! {
        <div class={classes!("element-item", is_selected.then(|| "selected"))}>
            <div class="element-header" onclick={onclick}>
                if !element.children.is_empty() {
                    <button class="expand-button" onclick={toggle_expanded}>
                        if *is_expanded {
                            {"▼"}
                        } else {
                            {"▶"}
                        }
                    </button>
                }
                <span class="element-name">{&element.name}</span>
            </div>
            if *is_expanded {
                <div class="element-children">
                    {for element.children.iter().map(|child| {
                        html! {
                            <ElementItem
                                element={child.clone()}
                                selected_id={selected_id.clone()}
                                on_select={on_select.clone()}
                                on_reparent={on_reparent.clone()}
                            />
                        }
                    })}
                </div>
            }
        </div>
    }
} 