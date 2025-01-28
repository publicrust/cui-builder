use yew::prelude::*;
use uuid::Uuid;
use crate::entities::cui_element::{
    CuiElement,
    components::{
        RectTransform,
        TextComponent,
        ImageComponent,
        CuiComponent
    }
};
use crate::entities::cui_container::CuiContainer;
use crate::components::canvas::CuiCanvas;
use crate::components::properties::panel::PropertiesPanel;

pub struct App {
    container: CuiContainer,
    selected_id: Option<String>,
}

pub enum Msg {
    AddRectangle,
    AddText,
    AddImage,
    SelectElement(String),
    UpdateElement(Box<dyn CuiComponent>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            container: CuiContainer::new(),
            selected_id: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddRectangle => {
                let id = Uuid::new_v4().to_string();
                let mut element = CuiElement::new(id.clone(), None);
                element.add_component(Box::new(RectTransform::default()));
                self.container.add_element(element);
                self.selected_id = Some(id);
                true
            }
            Msg::AddText => {
                let id = Uuid::new_v4().to_string();
                let mut element = CuiElement::new(id.clone(), None);
                element.add_component(Box::new(RectTransform::default()));
                element.add_component(Box::new(TextComponent::default()));
                self.container.add_element(element);
                self.selected_id = Some(id);
                true
            }
            Msg::AddImage => {
                let id = Uuid::new_v4().to_string();
                let mut element = CuiElement::new(id.clone(), None);
                element.add_component(Box::new(RectTransform::default()));
                element.add_component(Box::new(ImageComponent::default()));
                self.container.add_element(element);
                self.selected_id = Some(id);
                true
            }
            Msg::SelectElement(id) => {
                self.selected_id = Some(id);
                true
            }
            Msg::UpdateElement(component) => {
                if let Some(id) = &self.selected_id {
                    if let Some(element) = self.container.find_element_mut(id) {
                        let component_type = component.component_type();
                        if let Some(existing) = element.components.iter_mut()
                            .find(|c| c.component_type() == component_type) {
                            *existing = component;
                            return true;
                        }
                    }
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        
        html! {
            <div class="app">
                <div class="toolbar">
                    <button 
                        onclick={link.callback(|_| Msg::AddRectangle)}
                        class="button"
                        style="background-color: #409effff"
                    >
                        {"Add Rectangle"}
                    </button>
                    <button 
                        onclick={link.callback(|_| Msg::AddText)}
                        class="button"
                        style="background-color: #409effff"
                    >
                        {"Add Text"}
                    </button>
                    <button 
                        onclick={link.callback(|_| Msg::AddImage)}
                        class="button"
                        style="background-color: #409effff"
                    >
                        {"Add Image"}
                    </button>
                </div>
                <div class="workspace">
                    <div class="canvas">
                        <CuiCanvas
                            container={self.container.clone()}
                            selected_id={self.selected_id.clone()}
                            on_select={ctx.link().callback(Msg::SelectElement)}
                        />
                    </div>
                    <div class="properties">
                        <PropertiesPanel
                            selected_element={self.selected_id.as_ref().and_then(|id| self.container.find_element(id).cloned())}
                            on_element_change={ctx.link().callback(Msg::UpdateElement)}
                        />
                    </div>
                </div>
            </div>
        }
    }
} 