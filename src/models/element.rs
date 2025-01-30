use serde::{Deserialize, Serialize};
use crate::core::component::{Component, unity_canvas::UnityCanvasTransform};
use crate::oxide_interface::{
    elements::cui_element::CuiElement,
    components::{
        component_type::ComponentType,
        cui_rect_transform_component::CuiRectTransformComponent,
    },
};
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnityCanvasElement {
    pub id: String,
    pub name: String,
    pub transform: UnityCanvasTransform,
    pub elements: Vec<CuiElement>,
}

impl PartialEq for UnityCanvasElement {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id 
            && self.name == other.name 
            && self.transform == other.transform
            && self.elements == other.elements
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Element {
    pub id: String,
    pub name: String,
    pub parent: Option<String>,
    pub element_type: ElementType,
    pub components: Vec<Component>,
    pub fade_out: f32,
    pub destroy_ui: Option<String>,
}

impl From<CuiElement> for Element {
    fn from(cui: CuiElement) -> Self {
        // Определяем тип элемента по компонентам
        let element_type = cui.components.iter().find_map(|c| match c {
            ComponentType::Button(_) => Some(ElementType::Button),
            ComponentType::Text(_) => Some(ElementType::Text),
            _ => None,
        }).unwrap_or(ElementType::Panel);

        // Конвертируем компоненты
        let components = cui.components.into_iter()
            .map(|c| match c {
                ComponentType::RectTransform(t) => Component::RectTransform(t),
                ComponentType::Button(b) => Component::Button(b),
                ComponentType::Text(t) => Component::Text(t),
                ComponentType::Image(i) => Component::Image(i),
                ComponentType::RawImage(r) => Component::RawImage(r),
                ComponentType::NeedsCursor(n) => Component::NeedsCursor(n),
                ComponentType::NeedsKeyboard(n) => Component::NeedsKeyboard(n),
            })
            .collect();

        Self {
            id: cui.name.clone(),
            name: cui.name,
            parent: Some(cui.parent),
            element_type,
            components,
            fade_out: cui.fade_out,
            destroy_ui: cui.destroy_ui,
        }
    }
}

impl Into<CuiElement> for Element {
    fn into(self) -> CuiElement {
        // Конвертируем компоненты обратно
        let components = self.components.into_iter()
            .map(|c| match c {
                Component::RectTransform(t) => ComponentType::RectTransform(t),
                Component::Button(b) => ComponentType::Button(b),
                Component::Text(t) => ComponentType::Text(t),
                Component::Image(i) => ComponentType::Image(i),
                Component::RawImage(r) => ComponentType::RawImage(r),
                Component::NeedsCursor(n) => ComponentType::NeedsCursor(n),
                Component::NeedsKeyboard(n) => ComponentType::NeedsKeyboard(n),
                Component::UnityCanvasTransform(_) => ComponentType::RectTransform(CuiRectTransformComponent::default()),
            })
            .collect();

        CuiElement {
            name: self.name,
            parent: self.parent.unwrap_or_else(|| "Hud".to_string()),
            components,
            fade_out: self.fade_out,
            destroy_ui: self.destroy_ui,
        }
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id 
            && self.name == other.name 
            && self.parent == other.parent
            && self.element_type == other.element_type
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ElementType {
    Panel,
    Text,
    Button,
    Container,
}

impl fmt::Display for ElementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElementType::Panel => write!(f, "Panel"),
            ElementType::Text => write!(f, "Text"),
            ElementType::Button => write!(f, "Button"),
            ElementType::Container => write!(f, "Container"),
        }
    }
}

impl Default for ElementType {
    fn default() -> Self {
        Self::Panel
    }
} 