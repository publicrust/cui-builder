use web_sys::{MouseEvent, WheelEvent, TouchEvent, console};
use yew::prelude::*;
use crate::models::Element;
use crate::core::component::Component;
use wasm_bindgen::JsCast;
use super::element::UnityElement;

#[derive(Properties, PartialEq)]
pub struct InfiniteCanvasProps {
    pub elements: Vec<Element>,
    pub selected_id: Option<String>,
    pub on_select: Callback<String>,
    pub on_reparent: Callback<(String, Option<String>)>,
    pub on_update_component: Callback<(String, Component)>,
}

#[derive(Clone, PartialEq)]
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
    let last_mouse_pos = use_state(|| None::<(f64, f64)>);

    let onmousedown = {
        let is_panning = is_panning.clone();
        let last_mouse_pos = last_mouse_pos.clone();
        Callback::from(move |e: MouseEvent| {
            if e.button() == 2 {
                e.prevent_default();
                is_panning.set(true);
                last_mouse_pos.set(Some((e.client_x() as f64, e.client_y() as f64)));
            }
        })
    };

    let onmouseup = {
        let is_panning = is_panning.clone();
        Callback::from(move |_: MouseEvent| {
            is_panning.set(false);
        })
    };

    let onmousemove = {
        let is_panning = is_panning.clone();
        let last_mouse_pos = last_mouse_pos.clone();
        let workspace_state = workspace_state.clone();
        Callback::from(move |e: MouseEvent| {
            if *is_panning {
                if let Some((last_x, last_y)) = *last_mouse_pos {
                    let dx = e.client_x() as f64 - last_x;
                    let dy = e.client_y() as f64 - last_y;
                    let mut new_state = (*workspace_state).clone();
                    new_state.offset_x += dx;
                    new_state.offset_y += dy;
                    workspace_state.set(new_state);
                }
                last_mouse_pos.set(Some((e.client_x() as f64, e.client_y() as f64)));
            }
        })
    };

    let onwheel = {
        let workspace_state = workspace_state.clone();
        Callback::from(move |e: WheelEvent| {
            e.prevent_default();
            let mut new_state = (*workspace_state).clone();
            let delta = -e.delta_y() as f64;
            let zoom_factor = if delta > 0.0 { 1.1 } else { 0.9 };
            new_state.scale *= zoom_factor;
            workspace_state.set(new_state);
        })
    };

    let workspace_style = format!(
        "transform: translate({}px, {}px) scale({});",
        workspace_state.offset_x, workspace_state.offset_y, workspace_state.scale
    );

    // Вычисляем размер ячейки сетки в зависимости от масштаба
    let base_grid_size = 100.0; // Базовый размер ячейки
    let scale = workspace_state.scale;
    
    // Определяем, какие уровни сетки нужно показать
    let show_small = scale > 0.7;
    let show_medium = scale > 0.3 && scale <= 1.5;
    let show_large = scale <= 0.7;

    let grid_style = format!(
        "background-size: {}px {}px;",
        base_grid_size, base_grid_size
    );

    let canvas_class = if *is_panning {
        "infinite-canvas panning"
    } else {
        "infinite-canvas"
    };

    let zoom_in = {
        let workspace_state = workspace_state.clone();
        Callback::from(move |_| {
            let mut new_state = (*workspace_state).clone();
            new_state.scale *= 1.2;
            workspace_state.set(new_state);
        })
    };

    let zoom_out = {
        let workspace_state = workspace_state.clone();
        Callback::from(move |_| {
            let mut new_state = (*workspace_state).clone();
            new_state.scale /= 1.2;
            workspace_state.set(new_state);
        })
    };

    html! {
        <div
            class={canvas_class}
            onmousedown={onmousedown}
            onmouseup={onmouseup}
            onmousemove={onmousemove}
            onwheel={onwheel}
            oncontextmenu={Callback::from(|e: MouseEvent| e.prevent_default())}
        >
            <div class="grid-container">
                if show_small {
                    <div class="grid small" style={grid_style}></div>
                }
                if show_medium {
                    <div class="grid medium" style={format!("background-size: {}px {}px;", base_grid_size * 2.0, base_grid_size * 2.0)}></div>
                }
                if show_large {
                    <div class="grid large" style={format!("background-size: {}px {}px;", base_grid_size * 4.0, base_grid_size * 4.0)}></div>
                }
            </div>
            <div class="workspace" style={workspace_style}>
                {for props.elements.iter().map(|element| {
                    html! {
                        <UnityElement
                            element={element.clone()}
                            selected_id={props.selected_id.clone()}
                            on_select={props.on_select.clone()}
                            on_reparent={props.on_reparent.clone()}
                            on_update_component={props.on_update_component.clone()}
                        />
                    }
                })}
            </div>
            <div class="zoom-controls">
                <button class="zoom-button" onclick={zoom_in}>
                    {"+"}
                </button>
                <div class="zoom-level">
                    {format!("{}%", (workspace_state.scale * 100.0) as i32)}
                </div>
                <button class="zoom-button" onclick={zoom_out}>
                    {"-"}
                </button>
            </div>
        </div>
    }
} 