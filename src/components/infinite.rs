use yew::prelude::*;
use web_sys::{DragEvent, MouseEvent, WheelEvent};
use uuid::Uuid;
use crate::models::element::{Element, ElementType};
use crate::core::component::Component;
use crate::oxide_interface::{
    components::{
        cui_rect_transform_component::{CuiRectTransformComponent, CuiRectTransform},
        cui_button_component::CuiButtonComponent,
        cui_text_component::CuiTextComponent,
        cui_image_component::CuiImageComponent,
        cui_raw_image_component::CuiRawImageComponent,
    },
    elements::cui_element::CuiElement,
};
use super::unity::UnityCanvas;
use crate::components::toolbar::{Toolbar, Tool};

#[function_component(InfiniteCanvas)]
pub fn infinite_canvas(props: &InfiniteCanvasProps) -> Html {
    let panning = use_state(|| false);
    let scale = use_state(|| 1.0);
    let offset_x = use_state(|| 0.0);
    let offset_y = use_state(|| 0.0);
    let last_mouse_x = use_state(|| 0.0);
    let last_mouse_y = use_state(|| 0.0);
    let selected_tool = use_state(|| Tool::Select);

    // ... existing code ...

    let canvas_class = classes!(
        "infinite-canvas",
        (*panning).then_some("panning"),
        (*selected_tool == Tool::Select).then_some("select-tool")
    );

    let on_tool_change = {
        let selected_tool = selected_tool.clone();
        Callback::from(move |tool: Tool| {
            selected_tool.set(tool);
        })
    };

    html! {
        <div 
            class={canvas_class}
            oncontextmenu={Callback::from(|e: MouseEvent| e.prevent_default())}
            {onmousedown}
            {onmouseup}
            {onmousemove}
            {onmouseleave}
            {onwheel}
            {ondragover}
            {ondrop}
        >
            <Toolbar
                scale={*scale}
                on_zoom_in={on_zoom_in}
                on_zoom_out={on_zoom_out}
                on_zoom_reset={on_zoom_reset}
                on_add_element={props.on_add_element.clone()}
                on_tool_change={on_tool_change}
                selected_tool={(*selected_tool).clone()}
            />
            // ... rest of the code ...
        </div>
    }
} 