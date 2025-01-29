use serde::Serialize;
use crate::oxide_interface::components::{
    cui_image_component::CuiImageComponent,
    cui_raw_image_component::CuiRawImageComponent,
    cui_rect_transform_component::CuiRectTransformComponent,
    component_type::ComponentType,
    ICuiComponent,
};
use super::{cui_element::CuiElement, ICuiElement};

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct CuiPanel {
    #[serde(flatten)]
    pub base: CuiElement,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<CuiImageComponent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_image: Option<CuiRawImageComponent>,
    pub rect_transform: CuiRectTransformComponent,
}

impl CuiPanel {
    pub fn new(name: String, parent: String) -> Self {
        let rect_transform = CuiRectTransformComponent::default();
        let components = vec![ComponentType::RectTransform(rect_transform.clone())];

        Self {
            base: CuiElement::new(name, parent, components, 0.0),
            image: None,
            raw_image: None,
            rect_transform,
        }
    }

    pub fn set_image(&mut self, image: CuiImageComponent) {
        if self.image.is_some() {
            self.base.components.retain(|c| !matches!(c, ComponentType::Image(_)));
        }
        self.base.components.push(ComponentType::Image(image.clone()));
        self.image = Some(image);
    }

    pub fn set_raw_image(&mut self, raw_image: CuiRawImageComponent) {
        if self.raw_image.is_some() {
            self.base.components.retain(|c| !matches!(c, ComponentType::RawImage(_)));
        }
        self.base.components.push(ComponentType::RawImage(raw_image.clone()));
        self.raw_image = Some(raw_image);
    }
}

impl ICuiElement for CuiPanel {
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