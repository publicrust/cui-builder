use web_sys::HtmlInputElement;
use yew::prelude::*;
use crate::components::{Element, ElementType, Transform, RectTransform};

#[derive(Properties, PartialEq)]
pub struct PropertiesPanelProps {
    pub selected_element: Option<Element>,
    pub on_transform_change: Callback<(String, Transform)>,
    pub on_rect_transform_change: Callback<(String, RectTransform)>,
}

#[function_component(PropertiesPanel)]
pub fn properties_panel(props: &PropertiesPanelProps) -> Html {
    if let Some(element) = &props.selected_element {
        let id = element.id.clone();
        
        match element.element_type {
            ElementType::UnityCanvas => {
                if let Some(transform) = &element.transform {
                    let transform = transform.clone();
                    let transform_for_display = transform.clone();
                    let on_change = {
                        let on_transform_change = props.on_transform_change.clone();
                        let id = id.clone();
                        move |field: &str, value: f64| {
                            let mut new_transform = transform.clone();
                            match field {
                                "x" => new_transform.x = value,
                                "y" => new_transform.y = value,
                                "width" => new_transform.width = value,
                                "height" => new_transform.height = value,
                                _ => {}
                            }
                            on_transform_change.emit((id.clone(), new_transform));
                        }
                    };

                    html! {
                        <div class="properties-panel">
                            <h3>{"Свойства элемента"}</h3>
                            <div class="property-group">
                                <h4>{"Трансформация"}</h4>
                                <div class="property-row">
                                    <label>{"X:"}</label>
                                    <input 
                                        type="number"
                                        value={transform_for_display.x.to_string()}
                                        onchange={let on_change = on_change.clone(); 
                                            move |e: Event| {
                                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                                    if let Ok(value) = input.value().parse::<f64>() {
                                                        on_change("x", value);
                                                    }
                                                }
                                            }
                                        }
                                    />
                                </div>
                                <div class="property-row">
                                    <label>{"Y:"}</label>
                                    <input 
                                        type="number"
                                        value={transform_for_display.y.to_string()}
                                        onchange={let on_change = on_change.clone();
                                            move |e: Event| {
                                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                                    if let Ok(value) = input.value().parse::<f64>() {
                                                        on_change("y", value);
                                                    }
                                                }
                                            }
                                        }
                                    />
                                </div>
                                <div class="property-row">
                                    <label>{"Ширина:"}</label>
                                    <input 
                                        type="number"
                                        value={transform_for_display.width.to_string()}
                                        onchange={let on_change = on_change.clone();
                                            move |e: Event| {
                                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                                    if let Ok(value) = input.value().parse::<f64>() {
                                                        on_change("width", value);
                                                    }
                                                }
                                            }
                                        }
                                    />
                                </div>
                                <div class="property-row">
                                    <label>{"Высота:"}</label>
                                    <input 
                                        type="number"
                                        value={transform_for_display.height.to_string()}
                                        onchange={let on_change = on_change.clone();
                                            move |e: Event| {
                                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                                    if let Ok(value) = input.value().parse::<f64>() {
                                                        on_change("height", value);
                                                    }
                                                }
                                            }
                                        }
                                    />
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <div class="properties-panel empty">
                            {"Выберите элемент для редактирования"}
                        </div>
                    }
                }
            },
            _ => {
                if let Some(rect_transform) = &element.rect_transform {
                    let rect_transform = rect_transform.clone();
                    let rect_transform_for_display = rect_transform.clone();
                    let on_change = {
                        let on_rect_transform_change = props.on_rect_transform_change.clone();
                        let id = id.clone();
                        move |field: &str, value: f64| {
                            let mut new_rect_transform = rect_transform.clone();
                            match field {
                                "anchor_min_x" => new_rect_transform.anchor_min.x = value,
                                "anchor_min_y" => new_rect_transform.anchor_min.y = value,
                                "anchor_max_x" => new_rect_transform.anchor_max.x = value,
                                "anchor_max_y" => new_rect_transform.anchor_max.y = value,
                                "offset_min_x" => new_rect_transform.offset_min.x = value,
                                "offset_min_y" => new_rect_transform.offset_min.y = value,
                                "offset_max_x" => new_rect_transform.offset_max.x = value,
                                "offset_max_y" => new_rect_transform.offset_max.y = value,
                                _ => {}
                            }
                            on_rect_transform_change.emit((id.clone(), new_rect_transform));
                        }
                    };

                    html! {
                        <div class="properties-panel">
                            <h3>{"Свойства элемента"}</h3>
                            <div class="property-group">
                                <h4>{"Anchor Min"}</h4>
                                <div class="property-row">
                                    <label>{"X Y:"}</label>
                                    <input 
                                        type="text"
                                        value={format!("{} {}", rect_transform_for_display.anchor_min.x, rect_transform_for_display.anchor_min.y)}
                                        onchange={
                                            let on_change = on_change.clone();
                                            move |e: Event| {
                                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                                    let input_value = input.value();
                                                    let values: Vec<&str> = input_value.split_whitespace().collect();
                                                    if values.len() == 2 {
                                                        if let (Ok(x), Ok(y)) = (values[0].parse::<f64>(), values[1].parse::<f64>()) {
                                                            on_change("anchor_min_x", x);
                                                            on_change("anchor_min_y", y);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    />
                                </div>
                            </div>
                            <div class="property-group">
                                <h4>{"Anchor Max"}</h4>
                                <div class="property-row">
                                    <label>{"X Y:"}</label>
                                    <input 
                                        type="text"
                                        value={format!("{} {}", rect_transform_for_display.anchor_max.x, rect_transform_for_display.anchor_max.y)}
                                        onchange={
                                            let on_change = on_change.clone();
                                            move |e: Event| {
                                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                                    let input_value = input.value();
                                                    let values: Vec<&str> = input_value.split_whitespace().collect();
                                                    if values.len() == 2 {
                                                        if let (Ok(x), Ok(y)) = (values[0].parse::<f64>(), values[1].parse::<f64>()) {
                                                            on_change("anchor_max_x", x);
                                                            on_change("anchor_max_y", y);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    />
                                </div>
                            </div>
                            <div class="property-group">
                                <h4>{"Offset Min"}</h4>
                                <div class="property-row">
                                    <label>{"X Y:"}</label>
                                    <input 
                                        type="text"
                                        value={format!("{} {}", rect_transform_for_display.offset_min.x, rect_transform_for_display.offset_min.y)}
                                        onchange={
                                            let on_change = on_change.clone();
                                            move |e: Event| {
                                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                                    let input_value = input.value();
                                                    let values: Vec<&str> = input_value.split_whitespace().collect();
                                                    if values.len() == 2 {
                                                        if let (Ok(x), Ok(y)) = (values[0].parse::<f64>(), values[1].parse::<f64>()) {
                                                            on_change("offset_min_x", x);
                                                            on_change("offset_min_y", y);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    />
                                </div>
                            </div>
                            <div class="property-group">
                                <h4>{"Offset Max"}</h4>
                                <div class="property-row">
                                    <label>{"X Y:"}</label>
                                    <input 
                                        type="text"
                                        value={format!("{} {}", rect_transform_for_display.offset_max.x, rect_transform_for_display.offset_max.y)}
                                        onchange={
                                            let on_change = on_change.clone();
                                            move |e: Event| {
                                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                                    let input_value = input.value();
                                                    let values: Vec<&str> = input_value.split_whitespace().collect();
                                                    if values.len() == 2 {
                                                        if let (Ok(x), Ok(y)) = (values[0].parse::<f64>(), values[1].parse::<f64>()) {
                                                            on_change("offset_max_x", x);
                                                            on_change("offset_max_y", y);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    />
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <div class="properties-panel empty">
                            {"Выберите элемент для редактирования"}
                        </div>
                    }
                }
            }
        }
    } else {
        html! {
            <div class="properties-panel empty">
                {"Выберите элемент для редактирования"}
            </div>
        }
    }
} 