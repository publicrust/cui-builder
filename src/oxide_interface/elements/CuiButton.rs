use serde::Serialize;
use crate::oxide_interface::components::{
    CuiButtonComponent,
    CuiRectTransformComponent,
    CuiTextComponent,
    ICuiComponent,
};
use super::CuiElement;

#[derive(Serialize)]
pub struct CuiButton {
    #[serde(flatten)]
    pub base: CuiElement,
    pub button: CuiButtonComponent,
    pub rect_transform: CuiRectTransformComponent,
    pub text: CuiTextComponent,
}

impl CuiButton {
    pub fn new(name: String, parent: String) -> Self {
        let button = CuiButtonComponent::default();
        let rect_transform = CuiRectTransformComponent::default();
        let text = CuiTextComponent::default();

        let components = vec![
            Box::new(button.clone()),
            Box::new(rect_transform.clone()),
            Box::new(text.clone()),
        ];

        Self {
            base: CuiElement::new(name, parent, components, 0.0),
            button,
            rect_transform,
            text,
        }
    }
}

impl CuiElement for CuiButton {
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