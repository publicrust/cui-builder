use yew::prelude::*;
use web_sys::{MouseEvent, WheelEvent};

#[derive(Clone, PartialEq)]
pub struct WorkspaceState {
    pub offset_x: f64,
    pub offset_y: f64,
    pub scale: f64,
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
pub fn infinite_canvas() -> Html {
    let workspace_state = use_state(WorkspaceState::default);
    let is_panning = use_state(|| false);

    let onmousedown = {
        let is_panning = is_panning.clone();
        Callback::from(move |e: MouseEvent| {
            if e.button() == 1 || e.button() == 2 { // Средняя или правая кнопка мыши
                e.prevent_default();
                e.stop_propagation();
                is_panning.set(true);
            }
        })
    };

    let onmouseup = {
        let is_panning = is_panning.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            is_panning.set(false);
        })
    };

    let onmousemove = {
        let workspace_state = workspace_state.clone();
        let is_panning = is_panning.clone();
        Callback::from(move |e: MouseEvent| {
            if *is_panning {
                e.prevent_default();
                e.stop_propagation();
                
                let dx = e.movement_x() as f64 / workspace_state.scale;
                let dy = e.movement_y() as f64 / workspace_state.scale;
                
                workspace_state.set(WorkspaceState {
                    offset_x: workspace_state.offset_x + dx,
                    offset_y: workspace_state.offset_y + dy,
                    scale: workspace_state.scale,
                });
            }
        })
    };

    let oncontextmenu = Callback::from(|e: MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
    });

    let onwheel = {
        let workspace_state = workspace_state.clone();
        Callback::from(move |e: WheelEvent| {
            if e.ctrl_key() {
                e.prevent_default();
                e.stop_propagation();
                
                let delta = if e.delta_y() > 0.0 { 0.9 } else { 1.1 };
                let new_scale = (workspace_state.scale * delta).max(0.1).min(5.0);
                
                workspace_state.set(WorkspaceState {
                    scale: new_scale,
                    ..(*workspace_state)
                });
            }
        })
    };

    let zoom_in = {
        let workspace_state = workspace_state.clone();
        Callback::from(move |_| {
            let new_scale = (workspace_state.scale * 1.2).min(5.0);
            workspace_state.set(WorkspaceState {
                scale: new_scale,
                ..(*workspace_state)
            });
        })
    };

    let zoom_out = {
        let workspace_state = workspace_state.clone();
        Callback::from(move |_| {
            let new_scale = (workspace_state.scale * 0.8).max(0.1);
            workspace_state.set(WorkspaceState {
                scale: new_scale,
                ..(*workspace_state)
            });
        })
    };

    let workspace_style = format!(
        "transform: translate({}px, {}px) scale({});",
        workspace_state.offset_x,
        workspace_state.offset_y,
        workspace_state.scale
    );

    let scale_percentage = (workspace_state.scale * 100.0).round() as i32;

    let canvas_class = if *is_panning { "infinite-canvas panning" } else { "infinite-canvas" };

    html! {
        <div class={canvas_class}
            onmousemove={onmousemove}
            onmousedown={onmousedown}
            onmouseup={&onmouseup}
            onmouseleave={onmouseup}
            onwheel={onwheel}
            oncontextmenu={oncontextmenu}
        >
            <div class="workspace" style={workspace_style}>
            </div>
            <div class="zoom-controls">
                <button class="zoom-button" onclick={zoom_out}>{"−"}</button>
                <div class="zoom-level">{format!("{}%", scale_percentage)}</div>
                <button class="zoom-button" onclick={zoom_in}>{"+"}</button>
            </div>
        </div>
    }
} 