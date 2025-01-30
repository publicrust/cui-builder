use yew::prelude::*;
use web_sys::{DragEvent, MouseEvent, WheelEvent};
use uuid::Uuid;
use crate::models::element::{Element, ElementType, UnityCanvasElement};
use crate::core::component::{Component, unity_canvas::UnityCanvasTransform};
use crate::oxide_interface::{
    components::{
        cui_rect_transform_component::{CuiRectTransformComponent, CuiRectTransform},
        cui_button_component::CuiButtonComponent,
        cui_text_component::CuiTextComponent,
        cui_image_component::CuiImageComponent,
        cui_raw_image_component::CuiRawImageComponent,
    },
    elements::cui_element::CuiElement,
    CuiElementContainer,
    CuiHelper,
};
use super::unity::UnityCanvas;
use crate::components::toolbar::{Toolbar, Tool};

#[derive(Properties, Clone, PartialEq)]
pub struct InfiniteCanvasProps {
    pub elements: Vec<Element>,
    pub selected_id: Option<String>,
    pub on_select: Callback<String>,
    pub on_reparent: Callback<(String, String)>,
    pub on_update_component: Callback<(String, Component)>,
    pub on_add_element: Callback<Element>,
}

#[function_component(InfiniteCanvas)]
pub fn infinite_canvas(props: &InfiniteCanvasProps) -> Html {
    let selected_tool = use_state(|| Tool::Select);
    let scale = use_state(|| 1.0);
    let is_panning = use_state(|| false);
    let last_mouse_pos = use_state(|| None::<(i32, i32)>);
    let offset = use_state(|| (0.0, 0.0));

    let onmousedown = {
        let is_panning = is_panning.clone();
        let last_mouse_pos = last_mouse_pos.clone();
        Callback::from(move |e: MouseEvent| {
            if e.buttons() == 2 { // Правая кнопка мыши
                e.prevent_default();
                is_panning.set(true);
                last_mouse_pos.set(Some((e.client_x() as i32, e.client_y() as i32)));
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
        let offset = offset.clone();
        let last_mouse_pos = last_mouse_pos.clone();
        Callback::from(move |e: MouseEvent| {
            if *is_panning {
                if let Some((last_x, last_y)) = *last_mouse_pos {
                    let dx = e.client_x() as f64 - last_x as f64;
                    let dy = e.client_y() as f64 - last_y as f64;
                    offset.set((offset.0 + dx, offset.1 + dy));
                    last_mouse_pos.set(Some((e.client_x() as i32, e.client_y() as i32)));
                }
            }
        })
    };

    let onmouseleave = {
        let is_panning = is_panning.clone();
        Callback::from(move |_: MouseEvent| {
            is_panning.set(false);
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
                    web_sys::console::log_1(&format!("Dropped template: {}", template).into());
                    
                    let element = match template.as_str() {
                        "Tool::UnityCanvas" => {
                            web_sys::console::log_1(&"Creating UnityCanvas".into());
                            let canvas = UnityCanvasElement {
                                id: Uuid::new_v4().to_string(),
                                name: format!("UnityCanvas_{}", Uuid::new_v4()),
                                transform: UnityCanvasTransform {
                                    x: 0.0,
                                    y: 0.0,
                                    width: 800.0,
                                    height: 600.0,
                                },
                                elements: Vec::new(),
                            };
                            Element {
                                id: canvas.id.clone(),
                                name: canvas.name.clone(),
                                parent: Some("Hud".to_string()),
                                element_type: ElementType::Container,
                                components: vec![
                                    Component::UnityCanvasTransform(canvas.transform.clone())
                                ],
                                fade_out: 0.0,
                                destroy_ui: None,
                            }
                        },
                        "Tool::Panel" => {
                            web_sys::console::log_1(&"Creating Panel".into());
                            Element {
                                id: Uuid::new_v4().to_string(),
                                name: "Panel".to_string(),
                                parent: Some("Hud".to_string()),
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
                                    Component::Image(CuiImageComponent::default()),
                                ],
                                fade_out: 0.0,
                                destroy_ui: None,
                            }
                        },
                        "Tool::Button" => {
                            web_sys::console::log_1(&"Creating Button".into());
                            Element {
                                id: Uuid::new_v4().to_string(),
                                name: "Button".to_string(),
                                parent: Some("Hud".to_string()),
                                element_type: ElementType::Button,
                                components: vec![
                                    Component::RectTransform(CuiRectTransformComponent::default()),
                                    Component::Button(CuiButtonComponent::default()),
                                    Component::Text(CuiTextComponent::default()),
                                ],
                                fade_out: 0.0,
                                destroy_ui: None,
                            }
                        },
                        "Tool::Label" => {
                            web_sys::console::log_1(&"Creating Label".into());
                            Element {
                                id: Uuid::new_v4().to_string(),
                                name: "Label".to_string(),
                                parent: Some("Hud".to_string()),
                                element_type: ElementType::Text,
                                components: vec![
                                    Component::RectTransform(CuiRectTransformComponent::default()),
                                    Component::Text(CuiTextComponent::default()),
                                ],
                                fade_out: 0.0,
                                destroy_ui: None,
                            }
                        },
                        "Tool::Image" => {
                            web_sys::console::log_1(&"Creating Image".into());
                            Element {
                                id: Uuid::new_v4().to_string(),
                                name: "Image".to_string(),
                                parent: Some("Hud".to_string()),
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
                                    Component::Image(CuiImageComponent::default()),
                                ],
                                fade_out: 0.0,
                                destroy_ui: None,
                            }
                        },
                        "Tool::RawImage" => {
                            web_sys::console::log_1(&"Creating RawImage".into());
                            Element {
                                id: Uuid::new_v4().to_string(),
                                name: "RawImage".to_string(),
                                parent: Some("Hud".to_string()),
                                element_type: ElementType::Panel,
                                components: vec![
                                    Component::RectTransform(CuiRectTransformComponent::default()),
                                    Component::RawImage(CuiRawImageComponent::default()),
                                ],
                                fade_out: 0.0,
                                destroy_ui: None,
                            }
                        },
                        _ => {
                            web_sys::console::log_1(&format!("Unknown template: {}", template).into());
                            return;
                        }
                    };
                    web_sys::console::log_1(&"Emitting element".into());
                    on_add_element.emit(element);
                }
            }
        })
    };

    let transform_style = format!(
        "transform: translate({}px, {}px) scale({});",
        offset.0, offset.1, *scale
    );

    // Группируем элементы по UnityCanvas
    let mut unity_canvases = Vec::new();

    // Находим все UnityCanvas
    for element in props.elements.iter() {
        if element.element_type == ElementType::Container {
            if let Some(Component::UnityCanvasTransform(transform)) = element.components.first() {
                let canvas = UnityCanvasElement {
                    id: element.id.clone(),
                    name: element.name.clone(),
                    transform: transform.clone(),
                    elements: props.elements.iter()
                        .filter(|e| e.parent.as_ref().map_or(false, |p| p == &element.id))
                        .map(|e| e.clone().into())
                        .collect(),
                };
                unity_canvases.push(canvas);
            }
        }
    }

    let canvas_class = classes!(
        "infinite-canvas",
        (*is_panning).then_some("panning")
    );

    let on_zoom_in = {
        let scale = scale.clone();
        Callback::from(move |_| {
            scale.set((*scale * 1.2).min(5.0));
        })
    };

    let on_zoom_out = {
        let scale = scale.clone();
        Callback::from(move |_| {
            scale.set((*scale / 1.2).max(0.1));
        })
    };

    let on_zoom_reset = {
        let scale = scale.clone();
        Callback::from(move |_| {
            scale.set(1.0);
        })
    };

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
            <div class="canvas-content" style={transform_style}>
                {for unity_canvases.iter().map(|canvas| {
                    html! {
                        <UnityCanvas
                            canvas={canvas.clone()}
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