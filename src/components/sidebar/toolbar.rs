use yew::prelude::*;
use web_sys::DragEvent;
use crate::models::element::{Element, ElementType};
use crate::core::component::Component;
use crate::oxide_interface::components::{
    cui_rect_transform_component::{CuiRectTransformComponent, CuiRectTransform},
    cui_image_component::CuiImageComponent,
    cui_button_component::CuiButtonComponent,
    cui_text_component::CuiTextComponent,
};

#[derive(Clone, Debug)]
pub enum ElementTemplate {
    UnityCanvas,
    Panel,
    Button,
    Label,
    Image,
    RawImage,
}

#[derive(Properties, PartialEq)]
pub struct ToolbarProps {
    pub on_add_element: Callback<Element>,
}

#[function_component(Toolbar)]
pub fn toolbar(props: &ToolbarProps) -> Html {
    let ondragstart = |template: ElementTemplate| {
        let on_add_element = props.on_add_element.clone();
        Callback::from(move |e: DragEvent| {
            if let Some(dt) = e.data_transfer() {
                let _ = dt.set_data("text/plain", &format!("{:?}", template));
            }
        })
    };

    html! {
        <div class="element-tools">
            <div class="element-tools-header">
                <h3>{"Доступные элементы"}</h3>
            </div>
            <div class="element-tools-content">
                <div class="element-tool" draggable="true" ondragstart={ondragstart(ElementTemplate::UnityCanvas)}>
                    <span class="element-tool-icon">{"🖼️"}</span>
                    <span class="element-tool-name">{"UnityCanvas"}</span>
                </div>
                <div class="element-tool" draggable="true" ondragstart={ondragstart(ElementTemplate::Panel)}>
                    <span class="element-tool-icon">{"📦"}</span>
                    <span class="element-tool-name">{"Panel"}</span>
                </div>
                <div class="element-tool" draggable="true" ondragstart={ondragstart(ElementTemplate::Button)}>
                    <span class="element-tool-icon">{"🔘"}</span>
                    <span class="element-tool-name">{"Button"}</span>
                </div>
                <div class="element-tool" draggable="true" ondragstart={ondragstart(ElementTemplate::Label)}>
                    <span class="element-tool-icon">{"📝"}</span>
                    <span class="element-tool-name">{"Label"}</span>
                </div>
                <div class="element-tool" draggable="true" ondragstart={ondragstart(ElementTemplate::Image)}>
                    <span class="element-tool-icon">{"🖼️"}</span>
                    <span class="element-tool-name">{"Image"}</span>
                </div>
                <div class="element-tool" draggable="true" ondragstart={ondragstart(ElementTemplate::RawImage)}>
                    <span class="element-tool-icon">{"🎨"}</span>
                    <span class="element-tool-name">{"RawImage"}</span>
                </div>
            </div>
        </div>
    }
} 