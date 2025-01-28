use serde::{Serialize, Deserialize};
use yew::prelude::*;
use super::interface::{Component, Point2D};
use web_sys::HtmlInputElement;
use super::interface::CuiComponent;
use std::any::Any;
use std::rc::Rc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RectTransform {
    pub position: Point2D,
    pub size: Point2D,
    pub rotation: f64,
}

impl RectTransform {
    pub fn new() -> Self {
        Self {
            position: Point2D(0.0, 0.0),
            size: Point2D(100.0, 100.0),
            rotation: 0.0,
        }
    }

    pub fn render_properties(&self, on_update: Callback<Box<dyn CuiComponent>>) -> Html {
        let position = self.position.clone();
        let size = self.size.clone();
        let rotation = self.rotation;
        
        let on_update = on_update.reform(|c: Box<dyn CuiComponent>| {
            Box::new(c.downcast_ref::<RectTransform>().unwrap().clone()) as Box<dyn CuiComponent>
        });

        let on_position_change_x = {
            let on_update = on_update.clone();
            let position = position.clone();
            let size = size.clone();
            let callback = move |value: f64| {
                let new_transform = RectTransform {
                    position: Point2D(value, position.1),
                    size: size.clone(),
                    rotation,
                };
                on_update.emit(Box::new(new_transform) as Box<dyn CuiComponent>);
            };
            Callback::from(callback)
        };

        let on_position_change_y = {
            let on_update = on_update.clone();
            let position = position.clone();
            let size = size.clone();
            let callback = move |value: f64| {
                let new_transform = RectTransform {
                    position: Point2D(position.0, value),
                    size: size.clone(),
                    rotation,
                };
                on_update.emit(Box::new(new_transform) as Box<dyn CuiComponent>);
            };
            Callback::from(callback)
        };

        let on_size_change_x = {
            let on_update = on_update.clone();
            let position = position.clone();
            let size = size.clone();
            let callback = move |value: f64| {
                let new_transform = RectTransform {
                    position: position.clone(),
                    size: Point2D(value, size.1),
                    rotation,
                };
                on_update.emit(Box::new(new_transform) as Box<dyn CuiComponent>);
            };
            Callback::from(callback)
        };

        let on_size_change_y = {
            let on_update = on_update.clone();
            let position = position.clone();
            let size = size.clone();
            let callback = move |value: f64| {
                let new_transform = RectTransform {
                    position: position.clone(),
                    size: Point2D(size.0, value),
                    rotation,
                };
                on_update.emit(Box::new(new_transform) as Box<dyn CuiComponent>);
            };
            Callback::from(callback)
        };

        let on_rotation_change = {
            let on_update = on_update;
            let position = position.clone();
            let size = size.clone();
            let callback = move |value: f64| {
                let new_transform = RectTransform {
                    position: position.clone(),
                    size: size.clone(),
                    rotation: value,
                };
                on_update.emit(Box::new(new_transform) as Box<dyn CuiComponent>);
            };
            Callback::from(callback)
        };

        let position_for_html = position.clone();
        let size_for_html = size.clone();

        html! {
            <div class="property-group">
                <h4>{"Transform Properties"}</h4>
                <div class="property">
                    <label>{"Position"}</label>
                    <input 
                        type="number" 
                        value={position_for_html.0.to_string()} 
                        onchange={Callback::from(move |e: Event| {
                            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                if let Ok(value) = input.value().parse::<f64>() {
                                    on_position_change_x.emit(value);
                                }
                            }
                        })}
                    />
                    <input 
                        type="number" 
                        value={position_for_html.1.to_string()} 
                        onchange={Callback::from(move |e: Event| {
                            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                if let Ok(value) = input.value().parse::<f64>() {
                                    on_position_change_y.emit(value);
                                }
                            }
                        })}
                    />
                </div>
                <div class="property">
                    <label>{"Size"}</label>
                    <input 
                        type="number" 
                        value={size_for_html.0.to_string()} 
                        onchange={Callback::from(move |e: Event| {
                            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                if let Ok(value) = input.value().parse::<f64>() {
                                    on_size_change_x.emit(value);
                                }
                            }
                        })}
                    />
                    <input 
                        type="number" 
                        value={size_for_html.1.to_string()} 
                        onchange={Callback::from(move |e: Event| {
                            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                if let Ok(value) = input.value().parse::<f64>() {
                                    on_size_change_y.emit(value);
                                }
                            }
                        })}
                    />
                </div>
                <div class="property">
                    <label>{"Rotation"}</label>
                    <input 
                        type="number" 
                        value={rotation.to_string()} 
                        onchange={Callback::from(move |e: Event| {
                            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                if let Ok(value) = input.value().parse::<f64>() {
                                    on_rotation_change.emit(value);
                                }
                            }
                        })}
                    />
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