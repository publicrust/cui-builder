use crate::oxide_interface::components::{ICuiComponent, component_type::ComponentType};

pub trait ICuiElement {
    fn get_name(&self) -> &str;
    fn get_parent(&self) -> &str;
    fn get_fade_out(&self) -> f32;
    fn get_components(&self) -> &[ComponentType];
    fn get_destroy_ui(&self) -> Option<&str>;
} 