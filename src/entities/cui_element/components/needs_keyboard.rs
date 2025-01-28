use serde::{Serialize, Deserialize};
use super::interface::CuiComponent;
use std::any::Any;
use yew::{html, Callback, Html};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NeedsKeyboardComponent;

impl Default for NeedsKeyboardComponent {
    fn default() -> Self {
        Self
    }
}

#[typetag::serde]
impl CuiComponent for NeedsKeyboardComponent {
    fn component_type(&self) -> &'static str {
        "NeedsKeyboard"
    }

    fn render_properties(&self, _on_update: Callback<Box<dyn CuiComponent>>) -> Html {
        html! {
            <div class="property-group">
                <h4>{"Keyboard Interaction"}</h4>
                <div class="property-row">
                    <label>{"This component requires keyboard interaction"}</label>
                </div>
            </div>
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn CuiComponent> {
        Box::new(self.clone())
    }
} 