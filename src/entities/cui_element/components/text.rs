use serde::{Serialize, Deserialize};
use yew::prelude::*;
use super::interface::{CuiComponent, Color};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use std::any::Any;
use std::rc::Rc;
use yew::html::Html;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextComponent {
    #[serde(rename = "text")]
    pub text: String,
    
    #[serde(rename = "fontSize")]
    pub font_size: i32,
    
    #[serde(rename = "color")]
    pub color: Color,
    
    #[serde(rename = "font", skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    
    #[serde(rename = "align")]
    pub align: TextAlign,
    
    #[serde(rename = "outline", skip_serializing_if = "Option::is_none")]
    pub outline: Option<f32>,
    
    #[serde(rename = "outlineColor", skip_serializing_if = "Option::is_none")]
    pub outline_color: Option<Color>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum TextAlign {
    #[default]
    MiddleCenter,
    UpperLeft,
    UpperCenter,
    UpperRight,
    MiddleLeft,
    MiddleRight,
    LowerLeft,
    LowerCenter,
    LowerRight,
}

impl Default for TextComponent {
    fn default() -> Self {
        Self {
            text: String::new(),
            font_size: 14,
            color: Color::default(),
            font: None,
            align: TextAlign::MiddleCenter,
            outline: None,
            outline_color: None,
        }
    }
}

#[typetag::serde]
impl CuiComponent for TextComponent {
    fn component_type(&self) -> &'static str { "Text" }
    
    fn render_properties(&self, on_update: Callback<Box<dyn CuiComponent>>) -> Html {
        let component = Rc::new(self.clone());
        
        html! {
            <div class="property-group">
                <h4>{"Text Properties"}</h4>
                <div class="property">
                    <label>{"Content"}</label>
                    <textarea
                        value={component.text.clone()}
                        oninput={{
                            let on_update = on_update.clone();
                            let component = component.clone();
                            Callback::from(move |e: InputEvent| {
                                if let Some(input) = e.target_dyn_into::<HtmlTextAreaElement>() {
                                    let mut new_component = (*component).clone();
                                    new_component.text = input.value();
                                    on_update.emit(Box::new(new_component));
                                }
                            })
                        }}
                    />
                </div>
                <div class="property">
                    <label>{"Font Size"}</label>
                    <input 
                        type="number" 
                        value={component.font_size.to_string()} 
                        onchange={{
                            let on_update = on_update.clone();
                            let component = component.clone();
                            Callback::from(move |e: Event| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    let mut new_component = (*component).clone();
                                    new_component.font_size = input.value().parse::<i32>().unwrap();
                                    on_update.emit(Box::new(new_component));
                                }
                            })
                        }}
                    />
                </div>
                <div class="property">
                    <label>{"Text Color"}</label>
                    <input 
                        type="color" 
                        value={component.color.to_hex()} 
                        onchange={{
                            let on_update = on_update.clone();
                            let component = component.clone();
                            Callback::from(move |e: Event| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    let mut new_component = (*component).clone();
                                    new_component.color = Color::from_hex(&input.value());
                                    on_update.emit(Box::new(new_component));
                                }
                            })
                        }}
                    />
                </div>
            </div>
        }
    }
    
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn clone_box(&self) -> Box<dyn CuiComponent> { Box::new(self.clone()) }
} 