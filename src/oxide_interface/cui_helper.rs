use serde_json::{to_string, to_string_pretty, from_str, Result};
use crate::oxide_interface::CuiElementContainer;
use crate::oxide_interface::elements::cui_element::CuiElement;

// Имитируем часть функционала Oxide, как в C# 
// (AddUi, DestroyUi, GetGuid, и т.д.)
pub struct CuiHelper;

impl CuiHelper {
    pub fn to_json(container: &CuiElementContainer, formatted: bool) -> Result<String> {
        if formatted {
            to_string_pretty(container)
        } else {
            to_string(container)
        }
    }

    pub fn from_json(json: &str) -> Result<CuiElementContainer> {
        from_str(json)
    }

    pub fn generate_guid() -> String {
        uuid::Uuid::new_v4().simple().to_string()
    }

    // Заглушка для AddUi – в реальном плагине вы бы делали RPC вызов
    pub fn add_ui(container: &CuiElementContainer) -> bool {
        let json = Self::to_json(container, false).unwrap_or_default();
        println!("Simulating sending AddUI RPC with JSON:\n{}", json);
        true
    }

    pub fn destroy_ui(element_name: &str) -> bool {
        println!("Simulating DestroyUI RPC for element: {}", element_name);
        true
    }
}

// Дополнительные методы, аналогичные вашим C# SetColor / GetColor:
//
// Определим простую структуру Color 
#[derive(Debug, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn parse(rgba: &str) -> Option<Color> {
        let parts: Vec<&str> = rgba.split_whitespace().collect();
        if parts.len() == 4 {
            if let (Ok(r), Ok(g), Ok(b), Ok(a)) = (
                parts[0].parse::<f32>(),
                parts[1].parse::<f32>(),
                parts[2].parse::<f32>(),
                parts[3].parse::<f32>(),
            ) {
                return Some(Color { r, g, b, a });
            }
        }
        None
    }
}

pub trait ICuiColor {
    fn set_color(&mut self, color: &Color);
    fn get_color(&self) -> Option<Color>;
} 