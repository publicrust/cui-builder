use serde::Serialize;
use uuid::Uuid;
use super::elements::CuiElement;
use super::components::{
    button::CuiButton,
    label::CuiLabel,
    panel::CuiPanel,
    ICuiComponent,
};

#[derive(Serialize, Default)]
pub struct CuiElementContainer {
    pub elements: Vec<CuiElement>,
}

impl CuiElementContainer {
    pub fn add_button(&mut self, button: CuiButton, parent: &str, name: Option<&str>, destroy_ui: Option<&str>) -> String {
        let guid = name.map(|s| s.to_string()).unwrap_or_else(|| Uuid::new_v4().to_string());
        let element = CuiElement {
            name: guid.clone(),
            parent: parent.to_string(),
            destroy_ui: destroy_ui.map(|s| s.to_string()),
            components: vec![
                Box::new(button.button),
                Box::new(button.rect_transform),
            ],
            fade_out: button.fade_out,
            update: false,
        };
        self.elements.push(element);

        // Если в кнопке есть Text
        if let Some(text_component) = button.text {
            let text_elem = CuiElement {
                name: Uuid::new_v4().to_string(),
                parent: guid.clone(),
                destroy_ui: None,
                components: vec![Box::new(text_component)],
                fade_out: button.fade_out,
                update: false,
            };
            self.elements.push(text_elem);
        }
        guid
    }

    pub fn add_label(&mut self, label: CuiLabel, parent: &str, name: Option<&str>, destroy_ui: Option<&str>) -> String {
        let guid = name.map(|s| s.to_string()).unwrap_or_else(|| Uuid::new_v4().to_string());
        let element = CuiElement {
            name: guid.clone(),
            parent: parent.to_string(),
            destroy_ui: destroy_ui.map(|s| s.to_string()),
            components: vec![
                Box::new(label.text),
                Box::new(label.rect_transform),
            ],
            fade_out: label.fade_out,
            update: false,
        };
        self.elements.push(element);
        guid
    }

    pub fn add_panel(&mut self, panel: CuiPanel, parent: &str, name: Option<&str>, destroy_ui: Option<&str>) -> String {
        let guid = name.map(|s| s.to_string()).unwrap_or_else(|| Uuid::new_v4().to_string());
        
        let mut comps: Vec<Box<dyn ICuiComponent>> = Vec::new();
        if let Some(img) = panel.image {
            comps.push(Box::new(img));
        }
        if let Some(raw_img) = panel.raw_image {
            comps.push(Box::new(raw_img));
        }
        comps.push(Box::new(panel.rect_transform));
        
        if panel.cursor_enabled {
            comps.push(Box::new(super::components::needs_cursor::CuiNeedsCursorComponent::default()));
        }
        if panel.keyboard_enabled {
            comps.push(Box::new(super::components::needs_keyboard::CuiNeedsKeyboardComponent::default()));
        }

        let element = CuiElement {
            name: guid.clone(),
            parent: parent.to_string(),
            destroy_ui: destroy_ui.map(|s| s.to_string()),
            components: comps,
            fade_out: panel.fade_out,
            update: false,
        };
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