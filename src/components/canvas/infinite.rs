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

#[derive(Properties, Clone, PartialEq)]
pub struct InfiniteCanvasProps {
    pub elements: Vec<CuiElement>,
    pub selected_id: Option<String>,
    pub on_select: Callback<String>,
    pub on_reparent: Callback<(String, String)>,
    pub on_update_component: Callback<(String, Component)>,
    pub on_add_element: Callback<Element>,
}

#[function_component(InfiniteCanvas)]
pub fn infinite_canvas(props: &InfiniteCanvasProps) -> Html {
    let panning = use_state(|| false);
    let scale = use_state(|| 1.0);
    let offset_x = use_state(|| 0.0);
    let offset_y = use_state(|| 0.0);
    let last_mouse_x = use_state(|| 0.0);
    let last_mouse_y = use_state(|| 0.0);

    let onmousedown = {
        let panning = panning.clone();
        let last_mouse_x = last_mouse_x.clone();
        let last_mouse_y = last_mouse_y.clone();
        Callback::from(move |e: MouseEvent| {
            if e.buttons() == 2 { // Правая кнопка мыши
                e.prevent_default();
                panning.set(true);
                last_mouse_x.set(e.client_x() as f64);
                last_mouse_y.set(e.client_y() as f64);
            }
        })
    };

    let onmouseup = {
        let panning = panning.clone();
        Callback::from(move |_: MouseEvent| {
            panning.set(false);
        })
    };

    let onmousemove = {
        let panning = panning.clone();
        let offset_x = offset_x.clone();
        let offset_y = offset_y.clone();
        let last_mouse_x = last_mouse_x.clone();
        let last_mouse_y = last_mouse_y.clone();
        Callback::from(move |e: MouseEvent| {
            if *panning {
                let dx = e.client_x() as f64 - *last_mouse_x;
                let dy = e.client_y() as f64 - *last_mouse_y;
                offset_x.set(*offset_x + dx);
                offset_y.set(*offset_y + dy);
                last_mouse_x.set(e.client_x() as f64);
                last_mouse_y.set(e.client_y() as f64);
            }
        })
    };

    let onmouseleave = {
        let panning = panning.clone();
        Callback::from(move |_: MouseEvent| {
            panning.set(false);
        })
    };

    let onwheel = {
        let scale = scale.clone();
        Callback::from(move |e: WheelEvent| {
            e.prevent_default();
            let delta = if e.delta_y() > 0.0 { 0.9f64 } else { 1.1f64 };
            scale.set((*scale * delta).max(0.1f64).min(5.0f64));
        })
    };

    let ondragover = Callback::from(|e: DragEvent| {
        e.prevent_default();
    });

    let ondrop = {
        let on_add_element = props.on_add_element.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            if let Some(dt) = e.data_transfer() {
                if let Ok(template) = dt.get_data("text/plain") {
                    let element = match template.as_str() {
                        "UnityCanvas" => Element {
                            id: Uuid::new_v4().to_string(),
                            name: "Unity Canvas".to_string(),
                            parent: None,
                            element_type: ElementType::Panel,
                            components: vec![
                                Component::RectTransform(CuiRectTransformComponent {
                                    base: CuiRectTransform {
                                        anchormin: "0 0".to_string(),
                                        anchormax: "1 1".to_string(),
                                        offsetmin: "10 10".to_string(),
                                        offsetmax: "-10 -10".to_string(),
                                    }
                                }),
                            ],
                            fade_out: 0.0,
                            destroy_ui: None,
                        },
                        "Panel" => Element {
                            id: Uuid::new_v4().to_string(),
                            name: "Panel".to_string(),
                            parent: None,
                            element_type: ElementType::Panel,
                            components: vec![
                                Component::RectTransform(CuiRectTransformComponent::default()),
                                Component::Image(CuiImageComponent::default()),
                            ],
                            fade_out: 0.0,
                            destroy_ui: None,
                        },
                        "Button" => Element {
                            id: Uuid::new_v4().to_string(),
                            name: "Button".to_string(),
                            parent: None,
                            element_type: ElementType::Button,
                            components: vec![
                                Component::RectTransform(CuiRectTransformComponent::default()),
                                Component::Button(CuiButtonComponent::default()),
                                Component::Text(CuiTextComponent::default()),
                            ],
                            fade_out: 0.0,
                            destroy_ui: None,
                        },
                        "Label" => Element {
                            id: Uuid::new_v4().to_string(),
                            name: "Label".to_string(),
                            parent: None,
                            element_type: ElementType::Text,
                            components: vec![
                                Component::RectTransform(CuiRectTransformComponent::default()),
                                Component::Text(CuiTextComponent::default()),
                            ],
                            fade_out: 0.0,
                            destroy_ui: None,
                        },
                        "Image" => Element {
                            id: Uuid::new_v4().to_string(),
                            name: "Image".to_string(),
                            parent: None,
                            element_type: ElementType::Panel,
                            components: vec![
                                Component::RectTransform(CuiRectTransformComponent::default()),
                                Component::Image(CuiImageComponent::default()),
                            ],
                            fade_out: 0.0,
                            destroy_ui: None,
                        },
                        "RawImage" => Element {
                            id: Uuid::new_v4().to_string(),
                            name: "RawImage".to_string(),
                            parent: None,
                            element_type: ElementType::Panel,
                            components: vec![
                                Component::RectTransform(CuiRectTransformComponent::default()),
                                Component::RawImage(CuiRawImageComponent::default()),
                            ],
                            fade_out: 0.0,
                            destroy_ui: None,
                        },
                        _ => return,
                    };
                    on_add_element.emit(element);
                }
            }
        })
    };

    let transform_style = format!(
        "transform: translate({}px, {}px) scale({});",
        *offset_x, *offset_y, *scale
    );

    // Группируем элементы по их родителям
    let canvases = props.elements.iter()
        .filter(|e| e.parent == "Hud")
        .cloned()
        .collect::<Vec<_>>();

    let children = props.elements.iter()
        .filter(|e| e.parent != "Hud")
        .cloned()
        .collect::<Vec<_>>();

    let canvas_class = classes!(
        "infinite-canvas",
        (*panning).then_some("panning")
    );

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
            <div class="canvas-controls">
                <button class="zoom-in" onclick={
                    let scale = scale.clone();
                    Callback::from(move |_| {
                        let new_scale = (*scale * 1.2f64).min(5.0f64);
                        scale.set(new_scale);
                    })
                }>
                    {"+"}
                </button>
                <div class="zoom-level">
                    {format!("{}%", (*scale * 100.0f64).round())}
                </div>
                <button class="zoom-out" onclick={
                    let scale = scale.clone();
                    Callback::from(move |_| {
                        let new_scale = (*scale * 0.8f64).max(0.1f64);
                        scale.set(new_scale);
                    })
                }>
                    {"-"}
                </button>
                <button class="reset" onclick={
                    let scale = scale.clone();
                    Callback::from(move |_| scale.set(1.0f64))
                }>
                    {"Reset Zoom"}
                </button>
                <button class="reset" onclick={
                    let scale = scale.clone();
                    let offset_x = offset_x.clone();
                    let offset_y = offset_y.clone();
                    Callback::from(move |_| {
                        scale.set(1.0f64);
                        offset_x.set(0.0f64);
                        offset_y.set(0.0f64);
                    })
                }>
                    {"Reset View"}
                </button>
            </div>
            <div class="canvas-content" style={transform_style}>
                {for canvases.iter().map(|canvas| {
                    let canvas_children = children.iter()
                        .filter(|e| e.parent == canvas.name)
                        .cloned()
                        .collect::<Vec<_>>();

                    html! {
                        <UnityCanvas
                            elements={canvas_children}
                            selected_id={props.selected_id.clone()}
                            on_select={props.on_select.clone()}
                            on_reparent={props.on_reparent.clone()}
                            on_update_component={props.on_update_component.clone()}
                            on_add_element={props.on_add_element.clone()}
                        />
                    }
                })}
            </div>
        </div>
    }
} 