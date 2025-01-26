use yew::prelude::*;
use uuid::Uuid;
use super::{Element, ElementType};
use super::component::{RectTransformComponent, ImageComponent, UnityCanvasTransform, Component};
use web_sys::{DragEvent, console};

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub elements: Vec<Element>,
    pub selected_id: Option<String>,
    pub on_add_element: Callback<Element>,
    pub on_select: Callback<String>,
    pub on_reparent: Callback<(String, Option<String>)>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let add_unity_canvas = {
        let on_add_element = props.on_add_element.clone();
        Callback::from(move |_| {
            let new_element = Element {
                id: Uuid::new_v4().to_string(),
                name: "Новый канвас".to_string(),
                element_type: ElementType::UnityCanvas,
                components: vec![
                    Component::UnityCanvasTransform(UnityCanvasTransform {
                        x: 100.0,
                        y: 100.0,
                        width: 400.0,
                        height: 300.0,
                    })
                ],
                children: vec![],
            };
            on_add_element.emit(new_element);
        })
    };

    let add_panel = {
        let on_add_element = props.on_add_element.clone();
        Callback::from(move |_| {
            let new_element = Element {
                id: Uuid::new_v4().to_string(),
                name: "Новая панель".to_string(),
                element_type: ElementType::Panel,
                components: vec![
                    Component::RectTransform(RectTransformComponent {
                        anchor_min: (0.0, 0.0),
                        anchor_max: (1.0, 1.0),
                        offset_min: (10.0, 10.0),
                        offset_max: (-10.0, -10.0),
                    }),
                    Component::Image(ImageComponent {
                        sprite: None,
                        color: Some("1.0 1.0 1.0 0.5".to_string()),
                        material: None,
                    })
                ],
                children: vec![],
            };
            on_add_element.emit(new_element);
        })
    };

    html! {
        <div class="sidebar">
            <div class="sidebar-header">
                <h2>{"Элементы"}</h2>
                <div class="element-tools">
                    <button onclick={add_unity_canvas}>{"Unity Canvas"}</button>
                    <button onclick={add_panel}>{"Panel"}</button>
                </div>
            </div>
            <div class="element-list">
                {for props.elements.iter().map(|element| {
                    html! {
                        <ElementItem
                            element={element.clone()}
                            selected_id={props.selected_id.clone()}
                            on_select={props.on_select.clone()}
                            on_reparent={props.on_reparent.clone()}
                        />
                    }
                })}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ElementItemProps {
    pub element: Element,
    pub selected_id: Option<String>,
    pub on_select: Callback<String>,
    pub on_reparent: Callback<(String, Option<String>)>,
}

#[function_component(ElementItem)]
pub fn element_item(props: &ElementItemProps) -> Html {
    let element_header_class = {
        let mut classes = vec!["element-header"];
        if Some(props.element.id.clone()) == props.selected_id {
            classes.push("selected");
        }
        classes.join(" ")
    };

    html! {
        <div class="element-item">
            <div 
                class={element_header_class}
                draggable="true"
                ondragstart={{
                    let id = props.element.id.clone();
                    Callback::from(move |e: DragEvent| {
                        console::log_1(&format!("SIDEBAR: Начало перетаскивания элемента {}", id).into());
                        if let Some(data_transfer) = e.data_transfer() {
                            let _ = data_transfer.set_data("text/plain", &id);
                            console::log_1(&format!("SIDEBAR: Установлен data transfer с id={}", id).into());
                        }
                    })
                }}
                ondragover={{
                    let id = props.element.id.clone();
                    Callback::from(move |e: DragEvent| {
                        e.prevent_default();
                        console::log_1(&format!("SIDEBAR: Элемент над {}", id).into());
                    })
                }}
                ondrop={{
                    let on_reparent = props.on_reparent.clone();
                    let id = props.element.id.clone();
                    Callback::from(move |e: DragEvent| {
                        e.prevent_default();
                        console::log_1(&format!("SIDEBAR: Попытка бросить элемент на {}", id).into());
                        if let Some(data_transfer) = e.data_transfer() {
                            if let Ok(child_id) = data_transfer.get_data("text/plain") {
                                if child_id != id {
                                    console::log_1(&format!("SIDEBAR: Перемещение элемента {} в {}", child_id, id).into());
                                    on_reparent.emit((child_id, Some(id.clone())));
                                } else {
                                    console::log_1(&"SIDEBAR: Попытка бросить элемент на самого себя - игнорируется".into());
                                }
                            }
                        }
                    })
                }}
                onclick={{
                    let on_select = props.on_select.clone();
                    let id = props.element.id.clone();
                    Callback::from(move |_| {
                        console::log_1(&format!("SIDEBAR: Выбран элемент {}", id).into());
                        on_select.emit(id.clone());
                    })
                }}
            >
                <span class="element-icon">{get_element_icon(&props.element.element_type)}</span>
                <span class="element-name">{&props.element.name}</span>
            </div>
            <div class="element-list">
                {for props.element.children.iter().map(|child| {
                    html! {
                        <ElementItem
                            element={child.clone()}
                            selected_id={props.selected_id.clone()}
                            on_select={props.on_select.clone()}
                            on_reparent={props.on_reparent.clone()}
                        />
                    }
                })}
            </div>
        </div>
    }
}

fn get_element_icon(element_type: &ElementType) -> &'static str {
    match element_type {
        ElementType::UnityCanvas => "🎨",
        ElementType::Panel => "⬜",
        ElementType::Text => "📝",
        ElementType::Button => "🔘",
    }
} 