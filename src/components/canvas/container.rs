use yew::prelude::*;
use crate::CuiElement;
use crate::oxide_interface::components::cui_rect_transform_component::CuiRectTransformComponent;
use crate::oxide_interface::{
    CuiElementContainer,
    components::{
        ICuiComponent,
    },
};

#[derive(Properties, Clone, PartialEq)]
pub struct CuiElementContainerProps {
    pub container: CuiElementContainer,
    pub on_element_select: Callback<CuiElement>,
    pub on_container_update: Callback<CuiElementContainer>,
}

pub struct CuiElementContainerComponent {
    container: CuiElementContainer,
}

impl Component for CuiElementContainerComponent {
    type Message = ();
    type Properties = CuiElementContainerProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            container: ctx.props().container.clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let elements = self.container.elements.iter().map(|element| {
            let on_select = {
                let element = element.clone();
                let on_element_select = ctx.props().on_element_select.clone();
                Callback::from(move |_| {
                    on_element_select.emit(element.clone());
                })
            };

            html! {
                <div 
                    class="cui-element"
                    onclick={on_select}
                    style={self.get_element_style(element)}
                >
                    {self.render_element_content(element)}
                </div>
            }
        });

        html! {
            <div class="cui-container">
                { for elements }
            </div>
        }
    }
}

impl CuiElementContainerComponent {
    fn get_element_style(&self, element: &CuiElement) -> String {
        let rect_transform = element.components.iter()
            .find(|c| c.component_type() == "RectTransform")
            .and_then(|c| c.downcast_ref::<CuiRectTransformComponent>());

        if let Some(transform) = rect_transform {
            format!(
                "position: absolute; \
                 left: {}%; \
                 top: {}%; \
                 width: {}%; \
                 height: {}%;",
                transform.anchormin.split_whitespace().next().unwrap_or("0"),
                transform.anchormin.split_whitespace().nth(1).unwrap_or("0"),
                transform.anchormax.split_whitespace().next().unwrap_or("100"),
                transform.anchormax.split_whitespace().nth(1).unwrap_or("100"),
            )
        } else {
            String::from("position: absolute;")
        }
    }

    fn render_element_content(&self, element: &CuiElement) -> Html {
        // В зависимости от типа компонентов рендерим содержимое
        if element.components.iter().any(|c| c.component_type() == "UnityEngine.UI.Button") {
            html! { <button class="cui-button">{"Button"}</button> }
        } else if element.components.iter().any(|c| c.component_type() == "UnityEngine.UI.Text") {
            html! { <span class="cui-text">{"Text"}</span> }
        } else if element.components.iter().any(|c| c.component_type() == "UnityEngine.UI.Image") {
            html! { <div class="cui-image">{"Image"}</div> }
        } else if element.components.iter().any(|c| c.component_type() == "UnityEngine.UI.RawImage") {
            html! { <div class="cui-raw-image">{"Raw Image"}</div> }
        } else {
            html! { <div class="cui-panel">{"Panel"}</div> }
        }
    }
} 