use yew::prelude::*;
use crate::models::element::{Element, ElementType};
use crate::core::component::{Component, RectTransformComponent, ImageComponent, UnityCanvasTransform};

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
                    Component::RectTransform(RectTransformComponent {
                        anchor_min: (0.0, 0.0),
                        anchor_max: (1.0, 1.0),
                        offset_min: (0.1, 0.1),
                        offset_max: (0.9, 0.9),
                    }),
                    Component::Image(ImageComponent {
                        sprite: None,
                        color: Some("1.0,1.0,1.0,1.0".to_string()),
                        material: None,
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