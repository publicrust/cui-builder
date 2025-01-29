use serde::Serialize;
use crate::oxide_interface::components::{
    cui_text_component::CuiTextComponent,
    cui_rect_transform_component::CuiRectTransformComponent,
    component_type::ComponentType,
    ICuiComponent,
};
use super::{cui_element::CuiElement, ICuiElement};

// CuiLabel аналогичен структуре C# - содержит text + RectTransform + fade_out
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct CuiLabel {
    #[serde(flatten)]
    pub base: CuiElement,
    pub text: CuiTextComponent,
    pub rect_transform: CuiRectTransformComponent,
}

impl CuiLabel {
    pub fn new(name: String, parent: String) -> Self {
        let text = CuiTextComponent::default();
        let rect_transform = CuiRectTransformComponent::default();

        let components = vec![
            ComponentType::Text(text.clone()),
            ComponentType::RectTransform(rect_transform.clone()),
        ];

        Self {
            base: CuiElement::new(name, parent, components, 0.0),
            text,
            rect_transform,
        }
    }
}

impl ICuiElement for CuiLabel {
    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_parent(&self) -> &str {
        &self.base.parent
    }

    fn get_fade_out(&self) -> f32 {
        self.base.fade_out
    }

    fn get_components(&self) -> &[ComponentType] {
        &self.base.components
    }

    fn get_destroy_ui(&self) -> Option<&str> {
        self.base.get_destroy_ui()
    }
}

impl Default for CuiLabel {
    fn default() -> Self {
        Self::new("Default".to_string(), "Hud".to_string())
    }
} 