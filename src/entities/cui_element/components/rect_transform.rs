use serde::{Serialize, Deserialize};
use yew::prelude::*;
use web_sys::HtmlInputElement;
use super::interface::CuiComponent;
use std::any::Any;
use crate::shared::lib::types::{Point, Size};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RectTransform {
    pub position: Point,
    pub size: Size,
    pub anchors: (f64, f64, f64, f64), // min_x, min_y, max_x, max_y
    pub margins: (f64, f64, f64, f64), // left, top, right, bottom
    pub pivot: Point,
    pub rotation: f64,
    pub scale: Point,
}

impl Default for RectTransform {
    fn default() -> Self {
        Self {
            position: Point { x: 0.0, y: 0.0 },
            size: Size { width: 100.0, height: 100.0 },
            anchors: (0.0, 0.0, 0.0, 0.0),
            margins: (0.0, 0.0, 0.0, 0.0),
            pivot: Point { x: 0.5, y: 0.5 },
            rotation: 0.0,
            scale: Point { x: 1.0, y: 1.0 },
        }
    }
}

impl RectTransform {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render_properties(&self, on_update: Callback<Box<dyn CuiComponent>>) -> Html {
        let position = self.position.clone();
        let size = self.size.clone();
        let rotation = self.rotation;
        let anchors = self.anchors;
        let margins = self.margins;
        let pivot = self.pivot.clone();
        let scale = self.scale.clone();
        
        let on_update = on_update.reform(|c: Box<dyn CuiComponent>| {
            Box::new(c.downcast_ref::<RectTransform>().unwrap().clone()) as Box<dyn CuiComponent>
        });

        let on_position_change_x = {
            let on_update = on_update.clone();
            let new_transform = self.clone();
            Callback::from(move |value: f64| {
                let mut new_transform = new_transform.clone();
                new_transform.position.x = value;
                on_update.emit(Box::new(new_transform) as Box<dyn CuiComponent>);
            })
        };

        let on_position_change_y = {
            let on_update = on_update.clone();
            let new_transform = self.clone();
            Callback::from(move |value: f64| {
                let mut new_transform = new_transform.clone();
                new_transform.position.y = value;
                on_update.emit(Box::new(new_transform) as Box<dyn CuiComponent>);
            })
        };

        let on_size_change_width = {
            let on_update = on_update.clone();
            let new_transform = self.clone();
            Callback::from(move |value: f64| {
                let mut new_transform = new_transform.clone();
                new_transform.size.width = value;
                on_update.emit(Box::new(new_transform) as Box<dyn CuiComponent>);
            })
        };

        let on_size_change_height = {
            let on_update = on_update.clone();
            let new_transform = self.clone();
            Callback::from(move |value: f64| {
                let mut new_transform = new_transform.clone();
                new_transform.size.height = value;
                on_update.emit(Box::new(new_transform) as Box<dyn CuiComponent>);
            })
        };

        html! {
            <div class="properties-group">
                <div class="property">
                    <label>{"Position X:"}</label>
                    <input type="number" 
                        value={position.x.to_string()} 
                        onchange={Callback::from(move |e: Event| {
                            let on_position_change_x = on_position_change_x.clone();
                            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                if let Ok(value) = input.value().parse::<f64>() {
                                    on_position_change_x.emit(value);
                                }
                            }
                        })}
                    />
                </div>
                <div class="property">
                    <label>{"Position Y:"}</label>
                    <input type="number" 
                        value={position.y.to_string()} 
                        onchange={Callback::from(move |e: Event| {
                            let on_position_change_y = on_position_change_y.clone();
                            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                if let Ok(value) = input.value().parse::<f64>() {
                                    on_position_change_y.emit(value);
                                }
                            }
                        })}
                    />
                </div>
                <div class="property">
                    <label>{"Width:"}</label>
                    <input type="number" 
                        value={size.width.to_string()} 
                        onchange={Callback::from(move |e: Event| {
                            let on_size_change_width = on_size_change_width.clone();
                            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                if let Ok(value) = input.value().parse::<f64>() {
                                    on_size_change_width.emit(value);
                                }
                            }
                        })}
                    />
                </div>
                <div class="property">
                    <label>{"Height:"}</label>
                    <input type="number" 
                        value={size.height.to_string()} 
                        onchange={Callback::from(move |e: Event| {
                            let on_size_change_height = on_size_change_height.clone();
                            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                if let Ok(value) = input.value().parse::<f64>() {
                                    on_size_change_height.emit(value);
                                }
                            }
                        })}
                    />
                </div>
                <div class="property">
                    <label>{"Rotation:"}</label>
                    <input type="number" value={rotation.to_string()} />
                </div>
            </div>
        }
    }
}

#[typetag::serde]
impl CuiComponent for RectTransform {
    fn component_type(&self) -> &'static str { "RectTransform" }
    
    fn render_properties(&self, on_update: Callback<Box<dyn CuiComponent>>) -> Html {
        Self::render_properties(self, on_update)
    }
    
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn clone_box(&self) -> Box<dyn CuiComponent> { Box::new(self.clone()) }
} 