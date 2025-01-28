use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct RectTransformComponent {
    pub position: (f64, f64),
    pub size: (f64, f64),
    pub rotation: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct ImageComponent {
    pub sprite: Option<String>,
    pub color: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct TextComponent {
    pub text: String,
    pub font_size: f64,
    pub color: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct ButtonComponent {
    pub text: String,
    pub color: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Component {
    RectTransform(RectTransformComponent),
    Image(ImageComponent),
    Text(TextComponent),
    Button(ButtonComponent),
}

impl Component {
    pub fn component_type(&self) -> &'static str {
        match self {
            Component::RectTransform(_) => "RectTransform",
            Component::Image(_) => "Image",
            Component::Text(_) => "Text",
            Component::Button(_) => "Button",
        }
    }
} 