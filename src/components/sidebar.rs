use yew::prelude::*;
use super::{Element, ElementType, RectTransform, Vector2};
use uuid::Uuid;
use web_sys::DragEvent;

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub elements: Vec<Element>,
    pub on_select: Callback<String>,
    pub on_add_element: Callback<Element>,
    pub on_reparent: Callback<(String, Option<String>)>, // (element_id, new_parent_id)
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let add_container = {
        let on_add_element = props.on_add_element.clone();
        Callback::from(move |_| {
            let new_element = Element {
                id: Uuid::new_v4().to_string(),
                name: "Новый контейнер".to_string(),
                element_type: ElementType::Container,
                rect_transform: RectTransform::default(),
                children: vec![],
            };
            on_add_element.emit(new_element);
        })
    };

    let add_text = {
        let on_add_element = props.on_add_element.clone();
        Callback::from(move |_| {
            let new_element = Element {
                id: Uuid::new_v4().to_string(),
                name: "Новый текст".to_string(),
                element_type: ElementType::Text,
                rect_transform: RectTransform {
                    anchor_min: Vector2 { x: 0.4, y: 0.4 },
                    anchor_max: Vector2 { x: 0.6, y: 0.6 },
                    offset_min: Vector2 { x: 0.0, y: 0.0 },
                    offset_max: Vector2 { x: 0.0, y: 0.0 },
                },
                children: vec![],
            };
            on_add_element.emit(new_element);
        })
    };

    let add_image = {
        let on_add_element = props.on_add_element.clone();
        Callback::from(move |_| {
            let new_element = Element {
                id: Uuid::new_v4().to_string(),
                name: "Новое изображение".to_string(),
                element_type: ElementType::Image,
                rect_transform: RectTransform {
                    anchor_min: Vector2 { x: 0.35, y: 0.35 },
                    anchor_max: Vector2 { x: 0.65, y: 0.65 },
                    offset_min: Vector2 { x: 0.0, y: 0.0 },
                    offset_max: Vector2 { x: 0.0, y: 0.0 },
                },
                children: vec![],
            };
            on_add_element.emit(new_element);
        })
    };

    let add_button = {
        let on_add_element = props.on_add_element.clone();
        Callback::from(move |_| {
            let new_element = Element {
                id: Uuid::new_v4().to_string(),
                name: "Новая кнопка".to_string(),
                element_type: ElementType::Button,
                rect_transform: RectTransform {
                    anchor_min: Vector2 { x: 0.4, y: 0.4 },
                    anchor_max: Vector2 { x: 0.6, y: 0.6 },
                    offset_min: Vector2 { x: 0.0, y: 0.0 },
                    offset_max: Vector2 { x: 0.0, y: 0.0 },
                },
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
                    <button class="tool-button" onclick={add_container}>{"Контейнер"}</button>
                    <button class="tool-button" onclick={add_text}>{"Текст"}</button>
                    <button class="tool-button" onclick={add_image}>{"Изображение"}</button>
                    <button class="tool-button" onclick={add_button}>{"Кнопка"}</button>
                </div>
            </div>
            <div class="elements-tree">
                { render_elements(&props.elements, &props.on_select, &props.on_reparent, None) }
            </div>
        </div>
    }
}

fn render_elements(
    elements: &[Element],
    on_select: &Callback<String>,
    on_reparent: &Callback<(String, Option<String>)>,
    _parent_id: Option<String>,
) -> Html {
    html! {
        <ul class="element-list">
            {elements.iter().map(|element| {
                let id = element.id.clone();
                let on_click = {
                    let on_select = on_select.clone();
                    let id = id.clone();
                    Callback::from(move |_| {
                        on_select.emit(id.clone());
                    })
                };

                let on_dragstart = {
                    let id = id.clone();
                    Callback::from(move |e: DragEvent| {
                        e.data_transfer().unwrap().set_data("text/plain", &id).unwrap();
                        e.data_transfer().unwrap().set_effect_allowed("move");
                    })
                };

                let on_dragover = Callback::from(|e: DragEvent| {
                    e.prevent_default();
                    e.data_transfer().unwrap().set_drop_effect("move");
                });

                let on_drop = {
                    let on_reparent = on_reparent.clone();
                    let parent_id = id.clone();
                    Callback::from(move |e: DragEvent| {
                        e.prevent_default();
                        if let Ok(dragged_id) = e.data_transfer().unwrap().get_data("text/plain") {
                            if dragged_id != parent_id {
                                on_reparent.emit((dragged_id, Some(parent_id.clone())));
                            }
                        }
                    })
                };

                html! {
                    <li class="element-item" key={id.clone()}>
                        <div class="element-header"
                            draggable="true"
                            ondragstart={on_dragstart}
                            ondragover={on_dragover}
                            ondrop={on_drop}
                            onclick={on_click}
                        >
                            <span class="element-icon">
                                {match element.element_type {
                                    ElementType::Container => "📦",
                                    ElementType::Text => "📝",
                                    ElementType::Image => "🖼️",
                                    ElementType::Button => "🔘",
                                }}
                            </span>
                            <span class="element-name">{&element.name}</span>
                        </div>
                        if !element.children.is_empty() {
                            { render_elements(&element.children, on_select, on_reparent, Some(id.clone())) }
                        }
                    </li>
                }
            }).collect::<Html>()}
        </ul>
    }
} 