use web_sys::{MouseEvent, WheelEvent, DragEvent, TouchEvent, TouchList, console, Event};
use yew::prelude::*;
use crate::components::Element;
use crate::components::component::{RectTransformComponent, UnityCanvasTransform};
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct InfiniteCanvasProps {
    pub elements: Vec<Element>,
    pub selected_id: Option<String>,
    pub on_select: Callback<String>,
    pub on_reparent: Callback<(String, Option<String>)>,
}

struct WorkspaceState {
    offset_x: f64,
    offset_y: f64,
    scale: f64,
}

impl Default for WorkspaceState {
    fn default() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            scale: 1.0,
        }
    }
}

#[function_component(InfiniteCanvas)]
pub fn infinite_canvas(props: &InfiniteCanvasProps) -> Html {
    let workspace_state = use_state(WorkspaceState::default);
    let is_panning = use_state(|| false);
    let last_touch_distance = use_state(|| None::<f64>);
    
    let canvas_class = {
        let mut classes = vec!["infinite-canvas"];
        if *is_panning {
            classes.push("panning");
        }
        classes.join(" ")
    };

    let onmousedown = {
        let is_panning = is_panning.clone();
        Callback::from(move |e: MouseEvent| {
            console::log_1(&"Mouse down event".into());
            console::log_1(&format!("Button: {}", e.button()).into());
            if e.button() == 2 { // Right mouse button
                e.prevent_default();
                is_panning.set(true);
                console::log_1(&"Started panning".into());
            }
        })
    };
    
    let onmouseup = {
        let is_panning = is_panning.clone();
        Callback::from(move |_: MouseEvent| {
            if *is_panning {
                console::log_1(&"Stopped panning".into());
                is_panning.set(false);
            }
        })
    };
    
    let onmousemove = {
        let workspace_state = workspace_state.clone();
        let is_panning = is_panning.clone();
        Callback::from(move |e: MouseEvent| {
            if *is_panning {
                let scale = workspace_state.scale.max(0.1);
                let dx = e.movement_x() as f64 / scale;
                let dy = e.movement_y() as f64 / scale;
                console::log_1(&format!("Panning dx: {}, dy: {}, scale: {}", dx, dy, scale).into());
                workspace_state.set(WorkspaceState {
                    offset_x: workspace_state.offset_x + dx,
                    offset_y: workspace_state.offset_y + dy,
                    scale,
                });
            }
        })
    };
    
    let onwheel = {
        let workspace_state = workspace_state.clone();
        Callback::from(move |e: WheelEvent| {
            e.prevent_default();
            let delta = if e.ctrl_key() || e.get_modifier_state("Meta") {
                // Масштабирование при Ctrl/Cmd + колесо или жест тачпада
                if e.delta_y() > 0.0 { 0.9 } else { 1.1 }
            } else {
                // Обычная прокрутка колесом
                1.0
            };

            if delta != 1.0 {
                let new_scale = (workspace_state.scale * delta).max(0.1).min(10.0);
                console::log_1(&format!("Zooming with delta: {}, new scale: {}", delta, new_scale).into());
                workspace_state.set(WorkspaceState {
                    offset_x: workspace_state.offset_x,
                    offset_y: workspace_state.offset_y,
                    scale: new_scale,
                });
            }
        })
    };

    let zoom_in = {
        let workspace_state = workspace_state.clone();
        Callback::from(move |_| {
            let new_scale = (workspace_state.scale * 1.1).min(10.0);
            workspace_state.set(WorkspaceState {
                offset_x: workspace_state.offset_x,
                offset_y: workspace_state.offset_y,
                scale: new_scale,
            });
        })
    };

    let zoom_out = {
        let workspace_state = workspace_state.clone();
        Callback::from(move |_| {
            let new_scale = (workspace_state.scale * 0.9).max(0.1);
            workspace_state.set(WorkspaceState {
                offset_x: workspace_state.offset_x,
                offset_y: workspace_state.offset_y,
                scale: new_scale,
            });
        })
    };

    let zoom_level = format!("{}%", (workspace_state.scale * 100.0).round());

    let workspace_style = format!(
        "transform: translate({}px, {}px) scale({});",
        workspace_state.offset_x,
        workspace_state.offset_y,
        workspace_state.scale
    );

    let ontouchstart = {
        let last_touch_distance = last_touch_distance.clone();
        Callback::from(move |e: TouchEvent| {
            if e.touches().length() == 2 {
                e.prevent_default();
                let touch1 = e.touches().get(0).unwrap().unchecked_into::<web_sys::Touch>();
                let touch2 = e.touches().get(1).unwrap().unchecked_into::<web_sys::Touch>();
                let distance = (
                    (touch1.client_x() - touch2.client_x()).pow(2) as f64 +
                    (touch1.client_y() - touch2.client_y()).pow(2) as f64
                ).sqrt();
                last_touch_distance.set(Some(distance));
            }
        })
    };

    let ontouchmove = {
        let workspace_state = workspace_state.clone();
        let last_touch_distance = last_touch_distance.clone();
        Callback::from(move |e: TouchEvent| {
            if e.touches().length() == 2 {
                e.prevent_default();
                let touch1 = e.touches().get(0).unwrap().unchecked_into::<web_sys::Touch>();
                let touch2 = e.touches().get(1).unwrap().unchecked_into::<web_sys::Touch>();
                let distance = (
                    (touch1.client_x() - touch2.client_x()).pow(2) as f64 +
                    (touch1.client_y() - touch2.client_y()).pow(2) as f64
                ).sqrt();
                
                if let Some(last_distance) = *last_touch_distance {
                    let scale_delta = distance / last_distance;
                    let new_scale = (workspace_state.scale * scale_delta).max(0.1).min(10.0);
                    console::log_1(&format!("Touch scale: {}, new scale: {}", scale_delta, new_scale).into());
                    workspace_state.set(WorkspaceState {
                        offset_x: workspace_state.offset_x,
                        offset_y: workspace_state.offset_y,
                        scale: new_scale,
                    });
                }
                last_touch_distance.set(Some(distance));
            }
        })
    };

    let ontouchend = {
        let last_touch_distance = last_touch_distance.clone();
        Callback::from(move |_: TouchEvent| {
            last_touch_distance.set(None);
        })
    };

    html! {
        <>
            <div class={canvas_class}
                onmousedown={onmousedown}
                onmouseup={onmouseup}
                onmousemove={onmousemove}
                onwheel={onwheel}
                oncontextmenu={Callback::from(|e: MouseEvent| e.prevent_default())}
                ontouchstart={ontouchstart}
                ontouchmove={ontouchmove}
                ontouchend={ontouchend}
            >
                <div class="workspace" style={workspace_style}>
                    <div class="rulers">
                        <div class="ruler horizontal"></div>
                        <div class="ruler vertical"></div>
                    </div>
                    {for props.elements.iter().map(|element| {
                        let element_class = if Some(element.id.clone()) == props.selected_id {
                            "unity-canvas selected"
                        } else {
                            "unity-canvas"
                        };
                        
                        let onclick = {
                            let on_select = props.on_select.clone();
                            let id = element.id.clone();
                            Callback::from(move |_| {
                                on_select.emit(id.clone());
                            })
                        };
                        
                        let style = if let Some(transform) = element.components.iter()
                            .find(|c| c.component_type() == "UnityCanvasTransform")
                            .and_then(|c| match c {
                                Component::UnityCanvasTransform(t) => Some(t),
                                _ => None,
                            }) {
                            format!(
                                "left: {}px; top: {}px; width: {}px; height: {}px;",
                                transform.x,
                                transform.y,
                                transform.width,
                                transform.height
                            )
                        } else {
                            String::new()
                        };
                        
                        html! {
                            <div
                                class={element_class}
                                style={style}
                                onclick={onclick}
                                ondragover={Callback::from(|e: DragEvent| {
                                    e.prevent_default();
                                    console::log_1(&"CANVAS: Элемент над Unity Canvas".into());
                                })}
                                ondrop={{
                                    let on_reparent = props.on_reparent.clone();
                                    let id = element.id.clone();
                                    Callback::from(move |e: DragEvent| {
                                        e.prevent_default();
                                        console::log_1(&"CANVAS: Попытка бросить элемент на Unity Canvas".into());
                                        if let Some(data_transfer) = e.data_transfer() {
                                            if let Ok(child_id) = data_transfer.get_data("text/plain") {
                                                console::log_1(&format!("CANVAS: Добавление элемента {} на Canvas {}", child_id, id).into());
                                                on_reparent.emit((child_id, Some(id.clone())));
                                            } else {
                                                console::log_1(&"CANVAS: Не удалось получить id перетаскиваемого элемента".into());
                                            }
                                        } else {
                                            console::log_1(&"CANVAS: Нет data transfer".into());
                                        }
                                    })
                                }}
                            >
                                {for element.children.iter().map(|child| {
                                    console::log_1(&format!("CANVAS: Отрисовка дочернего элемента {} для Canvas {}", child.id, element.id).into());
                                    html! {
                                        <UnityElement
                                            element={child.clone()}
                                            selected_id={props.selected_id.clone()}
                                            on_select={props.on_select.clone()}
                                            on_reparent={props.on_reparent.clone()}
                                        />
                                    }
                                })}
                            </div>
                        }
                    })}
                </div>
            </div>
            <div class="zoom-controls">
                <button class="zoom-button" onclick={zoom_out}>
                    {"-"}
                </button>
                <div class="zoom-level">
                    {zoom_level}
                </div>
                <button class="zoom-button" onclick={zoom_in}>
                    {"+"}
                </button>
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct UnityElementProps {
    pub element: Element,
    pub selected_id: Option<String>,
    pub on_select: Callback<String>,
    pub on_reparent: Callback<(String, Option<String>)>,
}

#[function_component(UnityElement)]
pub fn unity_element(props: &UnityElementProps) -> Html {
    let is_drag_over = use_state(|| false);
    
    let element_class = {
        let mut classes = vec![
            if Some(props.element.id.clone()) == props.selected_id {
                "unity-element selected"
            } else {
                "unity-element"
            }
        ];
        
        if *is_drag_over {
            classes.push("drag-over");
        }
        
        console::log_1(&format!("Element {} class: {}", props.element.id, classes.join(" ")).into());
        classes.join(" ")
    };

    let ondragstart = {
        let id = props.element.id.clone();
        Callback::from(move |e: DragEvent| {
            console::log_1(&format!("DRAG START: Element {} начал перетаскиваться", id).into());
            if let Some(data_transfer) = e.data_transfer() {
                let _ = data_transfer.set_data("text/plain", &id);
                console::log_1(&format!("DRAG START: Установлен data transfer с id={}", id).into());
            }
        })
    };

    let ondragover = {
        let is_drag_over = is_drag_over.clone();
        let id = props.element.id.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            is_drag_over.set(true);
            console::log_1(&format!("DRAG OVER: Элемент находится над {}", id).into());
            if let Some(data_transfer) = e.data_transfer() {
                if let Ok(dragged_id) = data_transfer.get_data("text/plain") {
                    console::log_1(&format!("DRAG OVER: Перетаскивается элемент {} над {}", dragged_id, id).into());
                }
            }
        })
    };

    let ondragleave = {
        let is_drag_over = is_drag_over.clone();
        let id = props.element.id.clone();
        Callback::from(move |_: DragEvent| {
            is_drag_over.set(false);
            console::log_1(&format!("DRAG LEAVE: Элемент покинул зону {}", id).into());
        })
    };

    let ondrop = {
        let on_reparent = props.on_reparent.clone();
        let id = props.element.id.clone();
        let is_drag_over = is_drag_over.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();
            is_drag_over.set(false);
            console::log_1(&format!("DROP: Попытка бросить элемент на {}", id).into());
            
            if let Some(data_transfer) = e.data_transfer() {
                if let Ok(child_id) = data_transfer.get_data("text/plain") {
                    console::log_1(&format!("DROP: Получен id перетаскиваемого элемента: {}", child_id).into());
                    if child_id != id {
                        console::log_1(&format!("DROP: Выполняется reparent {} -> {}", child_id, id).into());
                        on_reparent.emit((child_id, Some(id.clone())));
                    } else {
                        console::log_1(&"DROP: Попытка бросить элемент на самого себя - игнорируется".into());
                    }
                } else {
                    console::log_1(&"DROP: Не удалось получить id перетаскиваемого элемента".into());
                }
            } else {
                console::log_1(&"DROP: Нет data transfer".into());
            }
        })
    };

    let onclick = {
        let on_select = props.on_select.clone();
        let id = props.element.id.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            console::log_1(&format!("CLICK: Выбран элемент {}", id).into());
            on_select.emit(id.clone());
        })
    };
    
    let style = if let Some(transform) = props.element.components.iter()
        .find(|c| c.component_type() == "RectTransform")
        .and_then(|c| match c {
            Component::RectTransform(t) => Some(t),
            _ => None,
        }) {
        format!(
            "position: absolute; left: {}%; top: {}%; right: {}%; bottom: {}%;",
            transform.anchor_min.0 * 100.0,
            transform.anchor_min.1 * 100.0,
            (1.0 - transform.anchor_max.0) * 100.0,
            (1.0 - transform.anchor_max.1) * 100.0
        )
    } else {
        String::new()
    };
    
    html! {
        <div
            class={element_class}
            style={style}
            onclick={onclick}
            draggable="true"
            ondragstart={ondragstart}
            ondragover={ondragover}
            ondragleave={ondragleave}
            ondrop={ondrop}
        >
            {for props.element.children.iter().map(|child| {
                html! {
                    <UnityElement
                        element={child.clone()}
                        selected_id={props.selected_id.clone()}
                        on_select={props.on_select.clone()}
                        on_reparent={props.on_reparent.clone()}
                    />
                }
            })}
        </div>
    }
} 