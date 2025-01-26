use serde::{Serialize, Deserialize};
use yew::prelude::*;
use web_sys::{HtmlInputElement, Event};
use crate::core::component::Component;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RectTransformComponent {
    pub anchor_min: (f64, f64),
    pub anchor_max: (f64, f64),
    pub offset_min: (f64, f64),
    pub offset_max: (f64, f64),
}

impl RectTransformComponent {
    fn create_number_input(
        &self,
        value: f64,
        min: Option<f64>,
        max: Option<f64>,
        step: f64,
        on_change: Callback<Event>,
    ) -> Html {
        let min_attr = min.map(|v| v.to_string());
        let max_attr = max.map(|v| v.to_string());
        
        html! {
            <input 
                type="number"
                min={min_attr}
                max={max_attr}
                step={step.to_string()}
                value={value.to_string()}
                onchange={on_change}
            />
        }
    }

    fn validate_anchor(value: f64) -> f64 {
        value.max(0.0).min(1.0)
    }

    fn create_value_update_callback(
        component: RectTransformComponent,
        field: &'static str,
        index: usize,
        on_update: Callback<Component>,
    ) -> Callback<Event> {
        Callback::from(move |e: Event| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    let validated_value = match field {
                        "anchor_min" | "anchor_max" => Self::validate_anchor(value),
                        _ => value,
                    };

                    let mut updated_component = component.clone();
                    match (field, index) {
                        ("anchor_min", 0) => updated_component.anchor_min.0 = validated_value,
                        ("anchor_min", 1) => updated_component.anchor_min.1 = validated_value,
                        ("anchor_max", 0) => updated_component.anchor_max.0 = validated_value,
                        ("anchor_max", 1) => updated_component.anchor_max.1 = validated_value,
                        ("offset_min", 0) => updated_component.offset_min.0 = validated_value,
                        ("offset_min", 1) => updated_component.offset_min.1 = validated_value,
                        ("offset_max", 0) => updated_component.offset_max.0 = validated_value,
                        ("offset_max", 1) => updated_component.offset_max.1 = validated_value,
                        _ => return,
                    }

                    on_update.emit(Component::RectTransform(updated_component));
                }
            }
        })
    }

    pub fn render_properties_with_callback(&self, on_update: Callback<Component>) -> Html {
        let component = self.clone();
        
        html! {
            <div class="property-group">
                <h4>{"Rect Transform"}</h4>
                
                <div class="property-row">
                    <label>{"Anchor Min"}</label>
                    {self.create_number_input(
                        component.anchor_min.0,
                        Some(0.0),
                        Some(1.0),
                        0.1,
                        Self::create_value_update_callback(component.clone(), "anchor_min", 0, on_update.clone())
                    )}
                    {self.create_number_input(
                        component.anchor_min.1,
                        Some(0.0),
                        Some(1.0),
                        0.1,
                        Self::create_value_update_callback(component.clone(), "anchor_min", 1, on_update.clone())
                    )}
                </div>

                <div class="property-row">
                    <label>{"Anchor Max"}</label>
                    {self.create_number_input(
                        component.anchor_max.0,
                        Some(0.0),
                        Some(1.0),
                        0.1,
                        Self::create_value_update_callback(component.clone(), "anchor_max", 0, on_update.clone())
                    )}
                    {self.create_number_input(
                        component.anchor_max.1,
                        Some(0.0),
                        Some(1.0),
                        0.1,
                        Self::create_value_update_callback(component.clone(), "anchor_max", 1, on_update.clone())
                    )}
                </div>

                <div class="property-row">
                    <label>{"Offset Min"}</label>
                    {self.create_number_input(
                        component.offset_min.0,
                        None,
                        None,
                        1.0,
                        Self::create_value_update_callback(component.clone(), "offset_min", 0, on_update.clone())
                    )}
                    {self.create_number_input(
                        component.offset_min.1,
                        None,
                        None,
                        1.0,
                        Self::create_value_update_callback(component.clone(), "offset_min", 1, on_update.clone())
                    )}
                </div>

                <div class="property-row">
                    <label>{"Offset Max"}</label>
                    {self.create_number_input(
                        component.offset_max.0,
                        None,
                        None,
                        1.0,
                        Self::create_value_update_callback(component.clone(), "offset_max", 0, on_update.clone())
                    )}
                    {self.create_number_input(
                        component.offset_max.1,
                        None,
                        None,
                        1.0,
                        Self::create_value_update_callback(component.clone(), "offset_max", 1, on_update.clone())
                    )}
                </div>
            </div>
        }
    }
} 