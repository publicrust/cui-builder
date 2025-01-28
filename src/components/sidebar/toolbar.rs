use yew::prelude::*;
use crate::entities::element::Element;
use crate::shared::lib::{utils::generate_id, component::Component, types::Color};
use crate::shared::ui::Button;

#[derive(Properties, Clone, PartialEq)]
pub struct ToolbarProps {
    pub on_add_element: Callback<Element>,
}

#[function_component(Toolbar)]
pub fn toolbar(props: &ToolbarProps) -> Html {
    let ToolbarProps { on_add_element } = props.clone();

    let add_rect = {
        let on_add_element = on_add_element.clone();
        Callback::from(move |_| {
            let id = generate_id();
            let mut element = Element::new(id.clone(), format!("Rectangle {}", id));
            element.add_component(Component::RectTransform(Default::default()));
            on_add_element.emit(element);
        })
    };

    let add_text = {
        let on_add_element = on_add_element.clone();
        Callback::from(move |_| {
            let id = generate_id();
            let mut element = Element::new(id.clone(), format!("Text {}", id));
            element.add_component(Component::RectTransform(Default::default()));
            element.add_component(Component::Text(Default::default()));
            on_add_element.emit(element);
        })
    };

    let add_image = {
        let on_add_element = on_add_element;
        Callback::from(move |_| {
            let id = generate_id();
            let mut element = Element::new(id.clone(), format!("Image {}", id));
            element.add_component(Component::RectTransform(Default::default()));
            element.add_component(Component::Image(Default::default()));
            on_add_element.emit(element);
        })
    };

    let button_color = Some(Color::new(64, 158, 255, 255)); // Голубой цвет для кнопок

    html! {
        <div class="toolbar">
            <Button text="Add Rectangle" onclick={add_rect} color={button_color.clone()} />
            <Button text="Add Text" onclick={add_text} color={button_color.clone()} />
            <Button text="Add Image" onclick={add_image} color={button_color} />
        </div>
    }
} 