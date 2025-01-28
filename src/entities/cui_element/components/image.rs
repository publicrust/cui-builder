use serde::{Serialize, Deserialize};
use yew::prelude::*;
use super::interface::{CuiComponent, Color};
use web_sys::HtmlInputElement;
use std::any::Any;
use std::rc::Rc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImageComponent {
    #[serde(rename = "sprite", skip_serializing_if = "Option::is_none")]
    pub sprite: Option<String>,
    
    #[serde(rename = "tint")]
    pub tint: Color,
    
    #[serde(rename = "material", skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
}

impl Default for ImageComponent {
    fn default() -> Self {
        Self {
            sprite: None,
            tint: Color::default(),
            material: None,
        }
    }
}

#[typetag::serde]
impl CuiComponent for ImageComponent {
    fn component_type(&self) -> &'static str {
        "Image"
    }

    fn render_properties(&self, on_update: Callback<Box<dyn CuiComponent>>) -> Html {
        let on_update = on_update.reform(|c: Box<dyn CuiComponent>| {
            Box::new(c.downcast_ref::<ImageComponent>().unwrap().clone()) as Box<dyn CuiComponent>
        });
        
        let component = Rc::new(self.clone());
        
        html! {
            <div class="property-group">
                <h4>{"Image Properties"}</h4>
                <div class="property">
                    <input 
                        type="color" 
                        value={self.tint.to_hex()}
                        onchange={{
                            let on_update = on_update.clone();
                            let component = component.clone();
                            Callback::from(move |e: Event| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    let mut new_component = (*component).clone();
                                    new_component.tint = Color::from_hex(&input.value());
                                    on_update.emit(Box::new(new_component) as Box<dyn CuiComponent>);
                                }
                            })
                        }}
                    />
                </div>
            </div>
        }
    }

    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn CuiComponent> {
        Box::new(self.clone())
    }
}

impl Color {
    // Реализация полностью удалена
} 