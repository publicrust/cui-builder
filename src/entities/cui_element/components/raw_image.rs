use serde::{Serialize, Deserialize};
use yew::prelude::*;
use super::interface::{CuiComponent, Color};
use web_sys::HtmlInputElement;
use std::any::Any;
use std::rc::Rc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RawImageComponent {
    #[serde(rename = "texture", skip_serializing_if = "Option::is_none")]
    pub texture: Option<String>,
    
    #[serde(rename = "color")]
    pub color: Color,
}

impl Default for RawImageComponent {
    fn default() -> Self {
        Self {
            texture: None,
            color: Color::default(),
        }
    }
}

#[typetag::serde]
impl CuiComponent for RawImageComponent {
    fn component_type(&self) -> &'static str { "RawImage" }
    
    fn render_properties(&self, on_update: Callback<Box<dyn CuiComponent>>) -> Html {
        let component = Rc::new(self.clone());
        
        html! {
            <div class="property-group">
                <h4>{"Raw Image Properties"}</h4>
                <div class="property">
                    <label>{"Texture Path"}</label>
                    <input 
                        type="text" 
                        value={component.texture.clone().unwrap_or_default()}
                        onchange={{
                            let on_update = on_update.clone();
                            let component = component.clone();
                            Callback::from(move |e: Event| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    let mut new_component = (*component).clone();
                                    new_component.texture = Some(input.value());
                                    on_update.emit(Box::new(new_component));
                                }
                            })
                        }}
                    />
                </div>
                <div class="property">
                    <label>{"Color"}</label>
                    <input 
                        type="color" 
                        value={component.color.to_hex()}
                        onchange={{
                            let on_update = on_update;
                            let component = component;
                            Callback::from(move |e: Event| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    let mut new_component = (*component).clone();
                                    new_component.color = Color::from_hex(&input.value()).into();
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