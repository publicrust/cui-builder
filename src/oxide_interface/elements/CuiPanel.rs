use serde::Serialize;
use crate::oxide_interface::components::{
    CuiImageComponent,
    CuiRawImageComponent,
    CuiRectTransformComponent,
    ICuiComponent,
};
use super::CuiElement;

#[derive(Serialize)]
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
        let mut components: Vec<Box<dyn ICuiComponent>> = Vec::new();
        let rect_transform = CuiRectTransformComponent::default();
        components.push(Box::new(rect_transform.clone()));

        Self {
            base: CuiElement::new(name, parent, components, 0.0),
            image: None,
            raw_image: None,
            rect_transform,
        }
    }

    pub fn set_image(&mut self, image: CuiImageComponent) {
        if let Some(old_image) = self.image.take() {
            self.base.components.retain(|c| c.component_type() != "UnityEngine.UI.Image");
        }
        self.base.components.push(Box::new(image.clone()));
        self.image = Some(image);
    }

    pub fn set_raw_image(&mut self, raw_image: CuiRawImageComponent) {
        if let Some(old_raw_image) = self.raw_image.take() {
            self.base.components.retain(|c| c.component_type() != "UnityEngine.UI.RawImage");
        }
        self.base.components.push(Box::new(raw_image.clone()));
        self.raw_image = Some(raw_image);
    }
} 