use yew::prelude::*;
use crate::models::element::{Element, ElementType};
use crate::core::component::{Component, UnityCanvasTransform};
use crate::oxide_interface::components::{
    cui_rect_transform_component::{CuiRectTransformComponent, CuiRectTransform},
    cui_image_component::CuiImageComponent,
};

#[derive(Properties, PartialEq)]
pub struct ToolbarProps {
    pub on_add_element: Callback<Element>,
}

#[function_component(Toolbar)]
pub fn toolbar(props: &ToolbarProps) -> Html {
    let on_add_unity_canvas = {
        let on_add_element = props.on_add_element.clone();
        Callback::from(move |_| {
            let element = Element {
                id: uuid::Uuid::new_v4().to_string(),
                name: "Unity Canvas".to_string(),
                element_type: ElementType::UnityCanvas,
                components: vec![
                    Component::UnityCanvasTransform(UnityCanvasTransform {
                        x: 0.0,
                        y: 0.0,
                        width: 800.0,
                        height: 600.0,
                    }),
                ],
                children: vec![],
            };
            on_add_element.emit(element);
        })
    };

    let on_add_panel = {
        let on_add_element = props.on_add_element.clone();
        Callback::from(move |_| {
            let element = Element {
                id: uuid::Uuid::new_v4().to_string(),
                name: "Panel".to_string(),
                element_type: ElementType::Panel,
                components: vec![
                    Component::RectTransform(CuiRectTransformComponent {
                        base: CuiRectTransform {
                            anchormin: "0 0".to_string(),
                            anchormax: "1 1".to_string(),
                            offsetmin: "10 10".to_string(),
                            offsetmax: "90 90".to_string(),
                        }
                    }),
                    Component::Image(CuiImageComponent {
                        color: Some("1 1 1 1".to_string()),
                        sprite: None,
                        material: None,
                        image_type: None,
                        png: None,
                        fade_in: None,
                        itemid: None,
                        skinid: None,
                    }),
                ],
                children: vec![],
            };
            on_add_element.emit(element);
        })
    };

    html! {
        <div class="toolbar">
            <button onclick={on_add_unity_canvas}>{"Add Unity Canvas"}</button>
            <button onclick={on_add_panel}>{"Add Panel"}</button>
        </div>
    }
} 