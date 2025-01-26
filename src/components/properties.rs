use yew::prelude::*;
use super::{Element, RectTransform, Vector2};

#[derive(Properties, PartialEq)]
pub struct PropertiesPanelProps {
    pub selected_element: Option<Element>,
    pub on_update_element: Callback<(String, RectTransform)>,
}

#[function_component(PropertiesPanel)]
pub fn properties_panel(props: &PropertiesPanelProps) -> Html {
    if let Some(element) = &props.selected_element {
        let id = element.id.clone();
        let on_update = props.on_update_element.clone();
        let rect_transform = element.rect_transform.clone();

        let create_vector2_handler = |field: &'static str| {
            let id = id.clone();
            let on_update = on_update.clone();
            let rect_transform = rect_transform.clone();
            
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let value = input.value();
                    let parts: Vec<&str> = value.split_whitespace().collect();
                    if parts.len() == 2 {
                        if let (Ok(x), Ok(y)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                            let mut new_transform = rect_transform.clone();
                            match field {
                                "anchor_min" => {
                                    new_transform.anchor_min.x = x;
                                    new_transform.anchor_min.y = y;
                                },
                                "anchor_max" => {
                                    new_transform.anchor_max.x = x;
                                    new_transform.anchor_max.y = y;
                                },
                                "offset_min" => {
                                    new_transform.offset_min.x = x;
                                    new_transform.offset_min.y = y;
                                },
                                "offset_max" => {
                                    new_transform.offset_max.x = x;
                                    new_transform.offset_max.y = y;
                                },
                                _ => {}
                            }
                            on_update.emit((id.clone(), new_transform));
                        }
                    }
                }
            })
        };

        let render_vector2_field = |label: &str, field: &'static str, vector: &Vector2| {
            html! {
                <div class="property-row">
                    <label>{label}</label>
                    <input 
                        type="text"
                        value={format!("{} {}", vector.x, vector.y)}
                        onchange={create_vector2_handler(field)}
                    />
                </div>
            }
        };

        html! {
            <div class="properties-panel">
                <h3>{"Свойства элемента"}</h3>
                <div class="property-group">
                    {render_vector2_field("AnchorMin:", "anchor_min", &rect_transform.anchor_min)}
                    {render_vector2_field("AnchorMax:", "anchor_max", &rect_transform.anchor_max)}
                    {render_vector2_field("OffsetMin:", "offset_min", &rect_transform.offset_min)}
                    {render_vector2_field("OffsetMax:", "offset_max", &rect_transform.offset_max)}
                </div>
            </div>
        }
    } else {
        html! {
            <div class="properties-panel empty">
                <p>{"Выберите элемент для редактирования свойств"}</p>
            </div>
        }
    }
} 