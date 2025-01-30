use yew::prelude::*;
use uuid::Uuid;
use crate::core::{
    element::Element,
    component::{Component, unity_transform::UnityTransform},
};

#[derive(Properties, Clone, PartialEq)]
pub struct ToolbarProps {
    pub on_add_element: Callback<Element>,
}

#[function_component(Toolbar)]
pub fn toolbar(props: &ToolbarProps) -> Html {
    let on_add_unity_canvas = {
        let on_add_element = props.on_add_element.clone();
        Callback::from(move |_| {
            let element = Element {
                id: Uuid::new_v4().to_string(),
                name: "Unity Canvas".to_string(),
                components: vec![Component::UnityTransform(UnityTransform::default())],
            };
            on_add_element.emit(element);
        })
    };

    html! {
        <div class="toolbar">
            <button onclick={on_add_unity_canvas}>{"Add Unity Canvas"}</button>
        </div>
    }
} 