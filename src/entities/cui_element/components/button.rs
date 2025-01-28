use serde::{Serialize, Deserialize};
use yew::prelude::*;
use super::interface::CuiComponent;
use std::any::Any;
use super::interface::Color;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ButtonComponent {
    pub text: String,
    #[serde(rename = "color")]
    pub color: Color,
}

impl Default for ButtonComponent {
    fn default() -> Self {
        Self {
            text: String::new(),
            color: Color { r: 0, g: 0, b: 0, a: 255 },
        }
    }
}

#[typetag::serde]
impl CuiComponent for ButtonComponent {
    fn component_type(&self) -> &'static str { "Button" }
    
    fn render_properties(&self, _on_update: Callback<Box<dyn CuiComponent>>) -> Html {
        html! { <div>{"Button Properties"}</div> }
    }
    
    fn clone_box(&self) -> Box<dyn CuiComponent> { 
        Box::new(self.clone()) 
    }
} 