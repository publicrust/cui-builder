use yew::prelude::*;
use web_sys::{MouseEvent, DragEvent};
use uuid::Uuid;
use crate::models::element::{Element, ElementType};
use crate::core::component::Component;
use crate::oxide_interface::components::{
    cui_rect_transform_component::{CuiRectTransformComponent, CuiRectTransform},
    cui_button_component::CuiButtonComponent,
    cui_text_component::CuiTextComponent,
    cui_image_component::CuiImageComponent,
    cui_raw_image_component::CuiRawImageComponent,
    component_type::ComponentType,
};
use crate::oxide_interface::{
    CuiElementContainer,
    elements::cui_element::CuiElement,
    CuiHelper,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Tool {
    Select,
    UnityCanvas,
    Panel,
    Button,
    Label,
    Image,
    RawImage,
}

#[derive(Properties, PartialEq)]
pub struct ToolbarProps {
    pub on_add_element: Callback<Element>,
    pub scale: f64,
    pub on_zoom_in: Callback<()>,
    pub on_zoom_out: Callback<()>,
    pub on_zoom_reset: Callback<()>,
    pub on_tool_change: Callback<Tool>,
    pub selected_tool: Tool,
}

#[function_component(Toolbar)]
pub fn toolbar(props: &ToolbarProps) -> Html {
    let on_tool_click = {
        let on_tool_change = props.on_tool_change.clone();
        move |tool: Tool| {
            let on_tool_change = on_tool_change.clone();
            Callback::from(move |_| {
                on_tool_change.emit(tool.clone());
            })
        }
    };

    let on_drag_start = {
        move |tool: Tool| {
            Callback::from(move |e: DragEvent| {
                if let Some(dt) = e.data_transfer() {
                    match tool {
                        Tool::UnityCanvas => {
                            let _ = dt.set_data("text/plain", "Tool::UnityCanvas");
                        },
                        _ => {
                            let tool_str = match tool {
                                Tool::Select => "Tool::Select",
                                Tool::Panel => "Tool::Panel",
                                Tool::Button => "Tool::Button",
                                Tool::Label => "Tool::Label",
                                Tool::Image => "Tool::Image",
                                Tool::RawImage => "Tool::RawImage",
                                _ => unreachable!(),
                            };
                            let _ = dt.set_data("text/plain", tool_str);
                        }
                    }
                }
            })
        }
    };

    let on_zoom_in = {
        let callback = props.on_zoom_in.clone();
        Callback::from(move |_: MouseEvent| callback.emit(()))
    };

    let on_zoom_out = {
        let callback = props.on_zoom_out.clone();
        Callback::from(move |_: MouseEvent| callback.emit(()))
    };

    let on_zoom_reset = {
        let callback = props.on_zoom_reset.clone();
        Callback::from(move |_: MouseEvent| callback.emit(()))
    };

    html! {
        <div class="tool-panel">
            <div class="tool-group">
                <button 
                    class={classes!("tool-button", (props.selected_tool == Tool::Select).then_some("active"))} 
                    onclick={on_tool_click(Tool::Select)}
                >
                    <span class="tool-icon">{"↖"}</span>
                </button>
                <button 
                    class={classes!("tool-button", (props.selected_tool == Tool::UnityCanvas).then_some("active"))} 
                    onclick={on_tool_click(Tool::UnityCanvas)}
                    draggable="true"
                    ondragstart={on_drag_start(Tool::UnityCanvas)}
                >
                    <span class="tool-icon">{"🖼️"}</span>
                </button>
                <button 
                    class={classes!("tool-button", (props.selected_tool == Tool::Panel).then_some("active"))} 
                    onclick={on_tool_click(Tool::Panel)}
                    draggable="true"
                    ondragstart={on_drag_start(Tool::Panel)}
                >
                    <span class="tool-icon">{"📦"}</span>
                </button>
                <button 
                    class={classes!("tool-button", (props.selected_tool == Tool::Button).then_some("active"))} 
                    onclick={on_tool_click(Tool::Button)}
                    draggable="true"
                    ondragstart={on_drag_start(Tool::Button)}
                >
                    <span class="tool-icon">{"🔘"}</span>
                </button>
                <button 
                    class={classes!("tool-button", (props.selected_tool == Tool::Label).then_some("active"))} 
                    onclick={on_tool_click(Tool::Label)}
                    draggable="true"
                    ondragstart={on_drag_start(Tool::Label)}
                >
                    <span class="tool-icon">{"📝"}</span>
                </button>
                <button 
                    class={classes!("tool-button", (props.selected_tool == Tool::Image).then_some("active"))} 
                    onclick={on_tool_click(Tool::Image)}
                    draggable="true"
                    ondragstart={on_drag_start(Tool::Image)}
                >
                    <span class="tool-icon">{"🖼️"}</span>
                </button>
            </div>
            <div class="tool-group zoom-controls">
                <button class="tool-button" onclick={on_zoom_out}>
                    <span class="tool-icon">{"-"}</span>
                </button>
                <span class="zoom-level">{format!("{}%", (props.scale * 100.0) as i32)}</span>
                <button class="tool-button" onclick={on_zoom_in}>
                    <span class="tool-icon">{"+"}</span>
                </button>
                <button class="tool-button" onclick={on_zoom_reset}>
                    <span class="tool-icon">{"↺"}</span>
                </button>
            </div>
        </div>
    }
} 