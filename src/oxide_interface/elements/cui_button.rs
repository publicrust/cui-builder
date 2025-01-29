use serde::Serialize;
use crate::oxide_interface::components::{
    cui_button_component::CuiButtonComponent,
    cui_rect_transform_component::CuiRectTransformComponent,
    cui_text_component::CuiTextComponent,
    component_type::ComponentType,
    ICuiComponent,
};
use super::{cui_element::CuiElement, ICuiElement};

#[derive(Serialize, Debug, Clone, PartialEq)]
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
            ComponentType::Button(button.clone()),
            ComponentType::RectTransform(rect_transform.clone()),
            ComponentType::Text(text.clone()),
        ];

        Self {
            base: CuiElement::new(name, parent, components, 0.0),
            button,
            rect_transform,
            text,
        }
    }
}

impl ICuiElement for CuiButton {
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