use serde::Serialize;
use crate::oxide_interface::components::{
    CuiTextComponent,
    CuiRectTransformComponent,
    ICuiComponent,
};
use super::CuiElement;

// CuiLabel аналогичен структуре C# - содержит text + RectTransform + fade_out
#[derive(Serialize)]
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
            Box::new(text.clone()),
            Box::new(rect_transform.clone()),
        ];

        Self {
            base: CuiElement::new(name, parent, components, 0.0),
            text,
            rect_transform,
        }
    }
}

impl CuiElement for CuiLabel {
    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_parent(&self) -> &str {
        &self.base.parent
    }

    fn get_fade_out(&self) -> f32 {
        self.base.fade_out
    }

    fn get_components(&self) -> Vec<Box<dyn ICuiComponent>> {
        self.base.components.clone()
    }

    fn get_destroy_ui(&self) -> Option<&str> {
        self.base.destroy_ui.as_deref()
    }
}

impl Default for CuiLabel {
    fn default() -> Self {
        Self::new()
    }
} 