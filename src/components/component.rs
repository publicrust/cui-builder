use serde::{Serialize, Deserialize};
use yew::prelude::*;
use std::any::Any;

#[typetag::serde(tag = "type")]
pub trait CuiComponent: std::fmt::Debug {
    fn component_type(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn render_properties(&self) -> Html;
    fn clone_box(&self) -> Box<dyn CuiComponent>;
}

impl Clone for Box<dyn CuiComponent> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RectTransformComponent {
    pub anchor_min: (f64, f64),
    pub anchor_max: (f64, f64),
    pub offset_min: (f64, f64),
    pub offset_max: (f64, f64),
}

#[typetag::serde]
impl CuiComponent for RectTransformComponent {
    fn component_type(&self) -> &'static str {
        "RectTransform"
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn render_properties(&self) -> Html {
        html! {
            <div class="property-group">
                <h4>{"Трансформация"}</h4>
                <div class="property-row">
                    <label>{"Anchor Min X:"}</label>
                    <input type="number" value={self.anchor_min.0.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Anchor Min Y:"}</label>
                    <input type="number" value={self.anchor_min.1.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Anchor Max X:"}</label>
                    <input type="number" value={self.anchor_max.0.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Anchor Max Y:"}</label>
                    <input type="number" value={self.anchor_max.1.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Offset Min X:"}</label>
                    <input type="number" value={self.offset_min.0.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Offset Min Y:"}</label>
                    <input type="number" value={self.offset_min.1.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Offset Max X:"}</label>
                    <input type="number" value={self.offset_max.0.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Offset Max Y:"}</label>
                    <input type="number" value={self.offset_max.1.to_string()} />
                </div>
            </div>
        }
    }

    fn clone_box(&self) -> Box<dyn CuiComponent> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageComponent {
    pub sprite: Option<String>,
    pub color: Option<String>,
    pub material: Option<String>,
}

#[typetag::serde]
impl CuiComponent for ImageComponent {
    fn component_type(&self) -> &'static str {
        "Image"
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn render_properties(&self) -> Html {
        html! {
            <div class="property-group">
                <h4>{"Изображение"}</h4>
                <div class="property-row">
                    <label>{"Спрайт:"}</label>
                    <input type="text" value={self.sprite.clone().unwrap_or_default()} />
                </div>
                <div class="property-row">
                    <label>{"Цвет:"}</label>
                    <input type="text" value={self.color.clone().unwrap_or_default()} />
                </div>
                <div class="property-row">
                    <label>{"Material:"}</label>
                    <input type="text" value={self.material.clone().unwrap_or_default()} />
                </div>
            </div>
        }
    }

    fn clone_box(&self) -> Box<dyn CuiComponent> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextComponent {
    pub text: String,
    pub font_size: f64,
    pub color: String,
    pub align: String,
}

#[typetag::serde]
impl CuiComponent for TextComponent {
    fn component_type(&self) -> &'static str {
        "Text"
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn render_properties(&self) -> Html {
        html! {
            <div class="property-group">
                <h4>{"Текст"}</h4>
                <div class="property-row">
                    <label>{"Текст:"}</label>
                    <input type="text" value={self.text.clone()} />
                </div>
                <div class="property-row">
                    <label>{"Размер шрифта:"}</label>
                    <input type="number" value={self.font_size.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Цвет:"}</label>
                    <input type="text" value={self.color.clone()} />
                </div>
                <div class="property-row">
                    <label>{"Выравнивание:"}</label>
                    <input type="text" value={self.align.clone()} />
                </div>
            </div>
        }
    }

    fn clone_box(&self) -> Box<dyn CuiComponent> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnityCanvasTransform {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[typetag::serde]
impl CuiComponent for UnityCanvasTransform {
    fn component_type(&self) -> &'static str {
        "UnityCanvasTransform"
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn render_properties(&self) -> Html {
        html! {
            <div class="property-group">
                <h4>{"Позиция и размер канваса"}</h4>
                <div class="property-row">
                    <label>{"X:"}</label>
                    <input type="number" value={self.x.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Y:"}</label>
                    <input type="number" value={self.y.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Ширина:"}</label>
                    <input type="number" value={self.width.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Высота:"}</label>
                    <input type="number" value={self.height.to_string()} />
                </div>
            </div>
        }
    }

    fn clone_box(&self) -> Box<dyn CuiComponent> {
        Box::new(self.clone())
    }
} 