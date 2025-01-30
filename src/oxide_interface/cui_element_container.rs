use serde::{Serialize, Deserialize};
use uuid::Uuid;
use super::elements::{
    cui_element::CuiElement,
    cui_button::CuiButton,
    cui_label::CuiLabel,
    cui_panel::CuiPanel,
    ICuiElement,
};
use super::components::{
    ICuiComponent,
    cui_needs_cursor_component::CuiNeedsCursorComponent,
    cui_needs_keyboard_component::CuiNeedsKeyboardComponent,
};
use crate::oxide_interface::{
    components::{
        cui_button_component::CuiButtonComponent,
        cui_image_component::CuiImageComponent,
        cui_raw_image_component::CuiRawImageComponent,
        cui_rect_transform_component::CuiRectTransformComponent,
        cui_text_component::CuiTextComponent,
        component_type::ComponentType,
    },
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CuiElementContainer {
    pub elements: Vec<CuiElement>,
}

impl CuiElementContainer {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, element: CuiElement) {
        self.elements.push(element);
    }

    pub fn add_button(&mut self, name: String, parent: String) -> String {
        let button = CuiButtonComponent::default();
        let rect_transform = CuiRectTransformComponent::default();
        let text = CuiTextComponent::default();

        let components = vec![
            ComponentType::Button(button),
            ComponentType::RectTransform(rect_transform),
            ComponentType::Text(text),
        ];

        let element = CuiElement::new(name.clone(), parent, components, 0.0);
        self.elements.push(element);
        name
    }

    pub fn add_panel(&mut self, name: String, parent: String) -> String {
        let rect_transform = CuiRectTransformComponent::default();
        let components = vec![ComponentType::RectTransform(rect_transform)];

        let element = CuiElement::new(name.clone(), parent, components, 0.0);
        self.elements.push(element);
        name
    }

    pub fn add_label(&mut self, name: String, parent: String) -> String {
        let text = CuiTextComponent::default();
        let rect_transform = CuiRectTransformComponent::default();

        let components = vec![
            ComponentType::Text(text),
            ComponentType::RectTransform(rect_transform),
        ];

        let element = CuiElement::new(name.clone(), parent, components, 0.0);
        self.elements.push(element);
        name
    }

    // Пример сериализации
    pub fn to_json(&self) -> String {
        match serde_json::to_string(self) {
            Ok(s) => s,
            Err(_) => String::new(),
        }
    }

    pub fn to_json_pretty(&self) -> String {
        match serde_json::to_string_pretty(self) {
            Ok(s) => s,
            Err(_) => String::new(),
        }
    }
}

impl Default for CuiElementContainer {
    fn default() -> Self {
        Self::new()
    }
} 