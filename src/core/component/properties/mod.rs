mod rect_transform;
mod unity_canvas_transform;
mod image;
mod text;
mod button;

pub use rect_transform::*;
pub use unity_canvas_transform::*;
pub use image::*;
pub use text::*;
pub use button::*;

use yew::prelude::*;
use crate::core::component::Component;

pub trait RenderProperties {
    fn render_properties_with_callback(&self, on_update: Callback<Component>) -> Html;
} 