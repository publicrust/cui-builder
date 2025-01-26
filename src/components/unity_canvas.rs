use yew::prelude::*;
use super::{Element, ElementType, RectTransform};
use web_sys::MouseEvent;

#[derive(Properties, PartialEq)]
pub struct UnityCanvasProps {
    pub elements: Vec<Element>,
    #[prop_or_default]
    pub selected_id: Option<String>,
    #[prop_or_default]
    pub on_element_move: Option<Callback<(String, RectTransform)>>,
    #[prop_or_default]
    pub on_select: Option<Callback<String>>,
}

#[derive(Default)]
struct DragState {
    is_dragging: bool,
    start_x: f64,
    start_y: f64,
    element_id: String,
    initial_transform: RectTransform,
}

#[function_component(UnityCanvas)]
pub fn unity_canvas(props: &UnityCanvasProps) -> Html {
    let drag_state = use_state(DragState::default);

    html! {
        <div class="unity-canvas-content">
            { render_elements(&props.elements, &props.selected_id, &props.on_element_move, &props.on_select, &drag_state) }
        </div>
    }
}

fn calculate_element_style(rect_transform: &RectTransform, parent_width: f64, parent_height: f64) -> String {
    let left = parent_width * rect_transform.anchor_min.x + rect_transform.offset_min.x;
    let top = parent_height * rect_transform.anchor_min.y + rect_transform.offset_min.y;
    let right = parent_width * rect_transform.anchor_max.x + rect_transform.offset_max.x;
    let bottom = parent_height * rect_transform.anchor_max.y + rect_transform.offset_max.y;

    let width = right - left;
    let height = bottom - top;

    format!(
        "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px;",
        left, top, width, height
    )
}

fn render_elements(
    elements: &[Element],
    selected_id: &Option<String>,
    on_element_move: &Option<Callback<(String, RectTransform)>>,
    on_select: &Option<Callback<String>>,
    drag_state: &UseStateHandle<DragState>,
) -> Html {
    let parent_width = 1000.0; // Фиксированный размер канваса для примера
    let parent_height = 800.0;

    html! {
        <>
            {elements.iter().map(|element| {
                let style = calculate_element_style(&element.rect_transform, parent_width, parent_height);
                let element_class = classes!(
                    "unity-element",
                    selected_id.as_ref().map(|id| id == &element.id).unwrap_or(false).then(|| "selected"),
                    match element.element_type {
                        ElementType::Container => "container-element",
                        ElementType::Text => "text-element",
                        ElementType::Image => "image-element",
                        ElementType::Button => "button-element",
                    }
                );

                let id = element.id.clone();
                let on_mousedown = {
                    let drag_state = drag_state.clone();
                    let element = element.clone();
                    let on_select = on_select.clone();
                    Callback::from(move |e: MouseEvent| {
                        if e.button() == 0 { // Только левая кнопка мыши
                            e.prevent_default();
                            e.stop_propagation();
                            
                            if let Some(on_select) = on_select.as_ref() {
                                on_select.emit(id.clone());
                            }
                            
                            drag_state.set(DragState {
                                is_dragging: true,
                                start_x: e.client_x() as f64,
                                start_y: e.client_y() as f64,
                                element_id: element.id.clone(),
                                initial_transform: element.rect_transform.clone(),
                            });
                        }
                    })
                };

                let onmousemove = {
                    let drag_state = drag_state.clone();
                    let on_element_move = on_element_move.clone();
                    Callback::from(move |e: MouseEvent| {
                        if drag_state.is_dragging {
                            e.prevent_default();
                            e.stop_propagation();
                            
                            let current_x = e.client_x() as f64;
                            let current_y = e.client_y() as f64;
                            let dx = current_x - drag_state.start_x;
                            let dy = current_y - drag_state.start_y;
                            
                            let mut new_transform = drag_state.initial_transform.clone();
                            new_transform.offset_min.x = drag_state.initial_transform.offset_min.x + dx;
                            new_transform.offset_min.y = drag_state.initial_transform.offset_min.y + dy;
                            new_transform.offset_max.x = drag_state.initial_transform.offset_max.x + dx;
                            new_transform.offset_max.y = drag_state.initial_transform.offset_max.y + dy;
                            
                            if let Some(on_element_move) = on_element_move.as_ref() {
                                on_element_move.emit((drag_state.element_id.clone(), new_transform));
                            }
                        }
                    })
                };

                let onmouseup = {
                    let drag_state = drag_state.clone();
                    Callback::from(move |e: MouseEvent| {
                        e.prevent_default();
                        e.stop_propagation();
                        drag_state.set(DragState::default());
                    })
                };

                html! {
                    <div
                        key={element.id.clone()}
                        class={element_class}
                        style={style}
                        onmousedown={on_mousedown}
                        onmousemove={onmousemove}
                        onmouseup={&onmouseup}
                        onmouseleave={onmouseup}
                    >
                        {match element.element_type {
                            ElementType::Container => html! {
                                <div class="container-content">
                                    { render_elements(&element.children, selected_id, on_element_move, on_select, drag_state) }
                                </div>
                            },
                            ElementType::Text => html! {
                                <div class="text-content">{"Текстовый элемент"}</div>
                            },
                            ElementType::Image => html! {
                                <div class="image-content">{"Изображение"}</div>
                            },
                            ElementType::Button => html! {
                                <div class="button-content">{"Кнопка"}</div>
                            },
                        }}
                    </div>
                }
            }).collect::<Html>()}
        </>
    }
} 