use yew::prelude::*;
use super::{Element, ElementType, Transform, RectTransform, Vector2};
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
    let add_unity_canvas = {
        let on_add_element = props.on_add_element.clone();
        Callback::from(move |_| {
            let new_element = Element {
                id: Uuid::new_v4().to_string(),
                name: "Новый канвас".to_string(),
                element_type: ElementType::UnityCanvas,
                transform: Some(Transform {
                    x: 0.0,
                    y: 0.0,
                    width: 400.0,
                    height: 300.0,
                }),
                rect_transform: None,
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
                transform: None,
                rect_transform: Some(RectTransform {
                    anchor_min: Vector2 { x: 0.0, y: 0.0 },
                    anchor_max: Vector2 { x: 1.0, y: 1.0 },
                    offset_min: Vector2 { x: 10.0, y: 10.0 },
                    offset_max: Vector2 { x: -10.0, y: -10.0 },
                }),
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
                transform: None,
                rect_transform: Some(RectTransform {
                    anchor_min: Vector2 { x: 0.5, y: 0.5 },
                    anchor_max: Vector2 { x: 0.5, y: 0.5 },
                    offset_min: Vector2 { x: -50.0, y: -25.0 },
                    offset_max: Vector2 { x: 50.0, y: 25.0 },
                }),
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
                transform: None,
                rect_transform: Some(RectTransform {
                    anchor_min: Vector2 { x: 0.5, y: 0.5 },
                    anchor_max: Vector2 { x: 0.5, y: 0.5 },
                    offset_min: Vector2 { x: -60.0, y: -20.0 },
                    offset_max: Vector2 { x: 60.0, y: 20.0 },
                }),
                children: vec![],
            };
            on_add_element.emit(new_element);
        })
    };

    fn render_elements_recursive(
        elements: &[Element],
        on_select: &Callback<String>,
        on_reparent: &Callback<(String, Option<String>)>,
        _parent_id: Option<String>,
    ) -> Html {
        elements.iter().map(|element| {
            let element_id = element.id.clone();
            let on_select = on_select.clone();
            let on_dragstart = {
                let element_id = element_id.clone();
                Callback::from(move |e: DragEvent| {
                    e.data_transfer().unwrap().set_data("text/plain", &element_id).unwrap();
                })
            };
            
            let on_dragover = Callback::from(|e: DragEvent| {
                e.prevent_default();
            });
            
            let on_drop = {
                let element_id = element_id.clone();
                let on_reparent = on_reparent.clone();
                Callback::from(move |e: DragEvent| {
                    e.prevent_default();
                    if let Ok(dragged_id) = e.data_transfer().unwrap().get_data("text/plain") {
                        on_reparent.emit((dragged_id, Some(element_id.clone())));
                    }
                })
            };

            let element_icon = match element.element_type {
                ElementType::UnityCanvas => "🎨",
                ElementType::Panel => "⬜",
                ElementType::Text => "📝",
                ElementType::Button => "🔘",
            };

            html! {
                <div class="element-item">
                    <div class="element-header"
                        draggable="true"
                        ondragstart={on_dragstart}
                        ondragover={on_dragover.clone()}
                        ondrop={on_drop}
                        onclick={let on_select = on_select.clone(); let id = element_id.clone(); move |_| on_select.emit(id.clone())}
                    >
                        <span class="element-icon">{element_icon}</span>
                        <span class="element-name">{&element.name}</span>
                    </div>
                    <div class="element-list">
                        {render_elements_recursive(&element.children, &on_select, &on_reparent, None)}
                    </div>
                </div>
            }
        }).collect::<Html>()
    }

    html! {
        <div class="sidebar">
            <div class="sidebar-header">
                <h2>{"Элементы"}</h2>
                <div class="element-tools">
                    <button class="tool-button" onclick={add_unity_canvas}>{"Unity Canvas"}</button>
                    <button class="tool-button" onclick={add_panel}>{"Panel"}</button>
                    <button class="tool-button" onclick={add_text}>{"Text"}</button>
                    <button class="tool-button" onclick={add_button}>{"Button"}</button>
                </div>
            </div>
            <div class="elements-tree">
                {render_elements_recursive(&props.elements, &props.on_select, &props.on_reparent, None)}
            </div>
        </div>
    }
} 