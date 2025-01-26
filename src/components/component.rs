use serde::{Serialize, Deserialize};
use yew::prelude::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Component {
    RectTransform(RectTransformComponent),
    UnityCanvasTransform(UnityCanvasTransform),
    Image(ImageComponent),
    Text(TextComponent),
    Button(ButtonComponent),
}

impl Component {
    pub fn component_type(&self) -> &'static str {
        match self {
            Component::RectTransform(_) => "RectTransform",
            Component::UnityCanvasTransform(_) => "UnityCanvasTransform",
            Component::Image(_) => "Image",
            Component::Text(_) => "Text",
            Component::Button(_) => "Button",
        }
    }

    pub fn render_properties(&self) -> Html {
        match self {
            Component::RectTransform(c) => c.render_properties(),
            Component::UnityCanvasTransform(c) => c.render_properties(),
            Component::Image(c) => c.render_properties(),
            Component::Text(c) => c.render_properties(),
            Component::Button(c) => c.render_properties(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RectTransformComponent {
    pub anchor_min: (f64, f64),
    pub anchor_max: (f64, f64),
    pub offset_min: (f64, f64),
    pub offset_max: (f64, f64),
}

impl RectTransformComponent {
    pub fn render_properties(&self) -> Html {
        html! {
            <div class="property-group">
                <h4>{"RectTransform"}</h4>
                <div class="property-row">
                    <label>{"Anchor Min"}</label>
                    <input type="number" value={self.anchor_min.0.to_string()} />
                    <input type="number" value={self.anchor_min.1.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Anchor Max"}</label>
                    <input type="number" value={self.anchor_max.0.to_string()} />
                    <input type="number" value={self.anchor_max.1.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Offset Min"}</label>
                    <input type="number" value={self.offset_min.0.to_string()} />
                    <input type="number" value={self.offset_min.1.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Offset Max"}</label>
                    <input type="number" value={self.offset_max.0.to_string()} />
                    <input type="number" value={self.offset_max.1.to_string()} />
                </div>
            </div>
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UnityCanvasTransform {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl UnityCanvasTransform {
    pub fn render_properties(&self) -> Html {
        html! {
            <div class="property-group">
                <h4>{"Unity Canvas Transform"}</h4>
                <div class="property-row">
                    <label>{"Position X"}</label>
                    <input type="number" value={self.x.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Position Y"}</label>
                    <input type="number" value={self.y.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Width"}</label>
                    <input type="number" value={self.width.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Height"}</label>
                    <input type="number" value={self.height.to_string()} />
                </div>
            </div>
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ImageComponent {
    pub sprite: Option<String>,
    pub color: Option<String>,
    pub material: Option<String>,
}

impl ImageComponent {
    pub fn render_properties(&self) -> Html {
        html! {
            <div class="property-group">
                <h4>{"Image"}</h4>
                <div class="property-row">
                    <label>{"Sprite"}</label>
                    <input type="text" value={self.sprite.clone().unwrap_or_default()} />
                </div>
                <div class="property-row">
                    <label>{"Color"}</label>
                    <input type="text" value={self.color.clone().unwrap_or_default()} />
                </div>
                <div class="property-row">
                    <label>{"Material"}</label>
                    <input type="text" value={self.material.clone().unwrap_or_default()} />
                </div>
            </div>
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TextComponent {
    pub text: String,
    pub font_size: f64,
    pub color: String,
    pub align: String,
}

impl TextComponent {
    pub fn render_properties(&self) -> Html {
        html! {
            <div class="property-group">
                <h4>{"Text"}</h4>
                <div class="property-row">
                    <label>{"Text"}</label>
                    <input type="text" value={self.text.clone()} />
                </div>
                <div class="property-row">
                    <label>{"Font Size"}</label>
                    <input type="number" value={self.font_size.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Color"}</label>
                    <input type="text" value={self.color.clone()} />
                </div>
                <div class="property-row">
                    <label>{"Align"}</label>
                    <input type="text" value={self.align.clone()} />
                </div>
            </div>
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ButtonComponent {
    pub text: String,
    pub font_size: f64,
    pub color: String,
    pub background_color: String,
}

impl ButtonComponent {
    pub fn render_properties(&self) -> Html {
        html! {
            <div class="property-group">
                <h4>{"Button"}</h4>
                <div class="property-row">
                    <label>{"Text"}</label>
                    <input type="text" value={self.text.clone()} />
                </div>
                <div class="property-row">
                    <label>{"Font Size"}</label>
                    <input type="number" value={self.font_size.to_string()} />
                </div>
                <div class="property-row">
                    <label>{"Text Color"}</label>
                    <input type="text" value={self.color.clone()} />
                </div>
                <div class="property-row">
                    <label>{"Background Color"}</label>
                    <input type="text" value={self.background_color.clone()} />
                </div>
            </div>
        }
    }
} 