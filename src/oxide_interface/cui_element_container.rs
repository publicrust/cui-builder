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

#[derive(Serialize, Deserialize, Default)]
pub struct CuiElementContainer {
    pub elements: Vec<CuiElement>,
}

impl CuiElementContainer {
    pub fn add_button(&mut self, button: CuiButton, parent: &str, name: Option<&str>, destroy_ui: Option<&str>) -> String {
        let guid = name.map(|s| s.to_string()).unwrap_or_else(|| Uuid::new_v4().to_string());
        let mut element = CuiElement::new(
            guid.clone(),
            parent.to_string(),
            vec![
                Box::new(button.button.clone()) as Box<dyn ICuiComponent>,
                Box::new(button.rect_transform.clone()) as Box<dyn ICuiComponent>,
                Box::new(button.text.clone()) as Box<dyn ICuiComponent>,
            ],
            0.0,
        );
        if let Some(destroy) = destroy_ui {
            element = element.with_destroy_ui(destroy.to_string());
        }
        self.elements.push(element);
        guid
    }

    pub fn add_label(&mut self, label: CuiLabel, parent: &str, name: Option<&str>, destroy_ui: Option<&str>) -> String {
        let guid = name.map(|s| s.to_string()).unwrap_or_else(|| Uuid::new_v4().to_string());
        let mut element = CuiElement::new(
            guid.clone(),
            parent.to_string(),
            vec![
                Box::new(label.text.clone()) as Box<dyn ICuiComponent>,
                Box::new(label.rect_transform.clone()) as Box<dyn ICuiComponent>,
            ],
            0.0,
        );
        if let Some(destroy) = destroy_ui {
            element = element.with_destroy_ui(destroy.to_string());
        }
        self.elements.push(element);
        guid
    }

    pub fn add_panel(&mut self, panel: CuiPanel, parent: &str, name: Option<&str>, destroy_ui: Option<&str>) -> String {
        let guid = name.map(|s| s.to_string()).unwrap_or_else(|| Uuid::new_v4().to_string());
        
        let mut comps: Vec<Box<dyn ICuiComponent>> = Vec::new();
        if let Some(img) = panel.image {
            comps.push(Box::new(img.clone()) as Box<dyn ICuiComponent>);
        }
        if let Some(raw_img) = panel.raw_image {
            comps.push(Box::new(raw_img.clone()) as Box<dyn ICuiComponent>);
        }
        comps.push(Box::new(panel.rect_transform.clone()) as Box<dyn ICuiComponent>);

        let mut element = CuiElement::new(
            guid.clone(),
            parent.to_string(),
            comps,
            0.0,
        );
        if let Some(destroy) = destroy_ui {
            element = element.with_destroy_ui(destroy.to_string());
        }
        self.elements.push(element);
        guid
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