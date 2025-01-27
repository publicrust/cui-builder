use yew::prelude::*;
use web_sys::{MouseEvent, Element as WebElement};
use crate::models::{Element, ElementType};
use crate::core::component::{Component, UnityCanvasTransform, RectTransformComponent};
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct UnityElementProps {
    pub element: Element,
    pub selected_id: Option<String>,
    pub on_select: Callback<String>,
    pub on_reparent: Callback<(String, Option<String>)>,
    pub on_update_component: Callback<(String, Component)>,
}

#[function_component(UnityElement)]
pub fn unity_element(props: &UnityElementProps) -> Html {
    let dragging = use_state(|| false);
    let initial_mouse_pos = use_state(|| None::<(f64, f64)>);
    let initial_element_pos = use_state(|| None::<(f64, f64)>);

    let onmousedown = {
        let dragging = dragging.clone();
        let initial_mouse_pos = initial_mouse_pos.clone();
        let initial_element_pos = initial_element_pos.clone();
        let element = props.element.clone();

        Callback::from(move |e: MouseEvent| {
            if e.button() == 0 {
                e.prevent_default();
                e.stop_propagation();
                dragging.set(true);
                
                initial_mouse_pos.set(Some((e.client_x() as f64, e.client_y() as f64)));
                
                // Получаем начальную позицию элемента из компонентов
                let initial_pos = if let Some(transform) = element.components.iter()
                    .find(|c| c.component_type() == "UnityCanvasTransform")
                {
                    if let Component::UnityCanvasTransform(t) = transform {
                        (t.x, t.y)
                    } else {
                        (0.0, 0.0)
                    }
                } else {
                    (0.0, 0.0)
                };
                
                initial_element_pos.set(Some(initial_pos));
            }
        })
    };

    let onmousemove = {
        let dragging = dragging.clone();
        let initial_mouse_pos = initial_mouse_pos.clone();
        let initial_element_pos = initial_element_pos.clone();
        let on_update_component = props.on_update_component.clone();
        let element_id = props.element.id.clone();
        let element = props.element.clone();
        
        Callback::from(move |e: MouseEvent| {
            if *dragging {
                if let (Some((start_x, start_y)), Some((init_x, init_y))) = (*initial_mouse_pos, *initial_element_pos) {
                    let dx = e.client_x() as f64 - start_x;
                    let dy = e.client_y() as f64 - start_y;
                    
                    // Получаем текущие размеры из компонента
                    if let Some(Component::UnityCanvasTransform(transform)) = element.components.iter()
                        .find(|c| c.component_type() == "UnityCanvasTransform")
                    {
                        // Создаем новый компонент с обновленными координатами
                        let new_transform = UnityCanvasTransform {
                            x: init_x + dx,
                            y: init_y + dy,
                            width: transform.width,
                            height: transform.height,
                        };
                        
                        on_update_component.emit((element_id.clone(), Component::UnityCanvasTransform(new_transform)));
                    }
                }
            }
        })
    };

    let onmouseup = {
        let dragging = dragging.clone();
        
        Callback::from(move |_: MouseEvent| {
            dragging.set(false);
            initial_mouse_pos.set(None);
            initial_element_pos.set(None);
        })
    };

    let element_class = classes!(
        "unity-element",
        props.selected_id.as_ref().map(|id| id == &props.element.id).unwrap_or(false).then_some("selected"),
        (*dragging).then_some("dragging")
    );

    let style = if props.element.element_type == ElementType::UnityCanvas {
        // Для UnityCanvas используем абсолютное позиционирование
        if let Some(transform) = props.element.components.iter()
            .find(|c| c.component_type() == "UnityCanvasTransform")
            .and_then(|c| match c {
                Component::UnityCanvasTransform(t) => Some(t),
                _ => None,
            }) {
            format!(
                "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px;",
                transform.x,
                transform.y,
                transform.width,
                transform.height
            )
        } else {
            String::new()
        }
    } else {
        // Для дочерних элементов используем anchor и offset с инвертированием по Y
        if let Some(transform) = props.element.components.iter()
            .find(|c| c.component_type() == "RectTransform")
            .and_then(|c| match c {
                Component::RectTransform(t) => Some(t),
                _ => None,
            }) {
            format!(
                "position: absolute; \
                left: {}%; \
                right: {}%; \
                bottom: {}%; \
                top: {}%; \
                margin-left: {}px; \
                margin-right: {}px; \
                margin-bottom: {}px; \
                margin-top: {}px;",
                transform.anchor_min.0 * 100.0,
                (1.0 - transform.anchor_max.0) * 100.0,
                transform.anchor_min.1 * 100.0, // Используем anchor_min.y для bottom
                (1.0 - transform.anchor_max.1) * 100.0, // Используем anchor_max.y для top
                transform.offset_min.0,
                -transform.offset_max.0,
                transform.offset_min.1, // Используем offset_min.y для margin-bottom
                -transform.offset_max.1 // Используем offset_max.y для margin-top
            )
        } else {
            String::new()
        }
    };

    html! {
        <div
            class={element_class}
            data-type={props.element.element_type.to_string()}
            style={style}
            onmousedown={onmousedown}
            onmousemove={onmousemove}
            onmouseup={onmouseup}
            onclick={
                let on_select = props.on_select.clone();
                let id = props.element.id.clone();
                Callback::from(move |e: MouseEvent| {
                    e.stop_propagation();
                    on_select.emit(id.clone());
                })
            }
        >
            {&props.element.name}
            {for props.element.children.iter().map(|child| {
                html! {
                    <UnityElement
                        element={child.clone()}
                        selected_id={props.selected_id.clone()}
                        on_select={props.on_select.clone()}
                        on_reparent={props.on_reparent.clone()}
                        on_update_component={props.on_update_component.clone()}
                    />
                }
            })}
        </div>
    }
} 