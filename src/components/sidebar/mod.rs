pub mod element_item;

use yew::prelude::*;
use crate::models::Element;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub elements: Vec<Element>,
    pub selected_id: Option<String>,
    pub on_select: Callback<String>,
    pub on_reparent: Callback<(String, String)>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &Props) -> Html {
    let Props {
        elements,
        selected_id,
        on_select,
        on_reparent,
    } = props;

    html! {
        <div class="sidebar">
            <div class="sidebar-header">
                <h2>{"Elements"}</h2>
            </div>
            <div class="sidebar-content">
                {
                    elements.iter().map(|element| {
                        let id = element.id.clone();
                        let on_select = on_select.clone();
                        let on_click = Callback::from(move |_| {
                            on_select.emit(id.clone());
                        });
                        
                        html! {
                            <div 
                                key={element.id.clone()}
                                class={classes!(
                                    "sidebar-item",
                                    selected_id.as_ref().map_or(false, |selected| selected == &element.id)
                                        .then_some("selected")
                                )}
                                onclick={on_click}
                            >
                                <span class="element-type">{&element.element_type.to_string()}</span>
                                <span class="element-name">{&element.name}</span>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
} 