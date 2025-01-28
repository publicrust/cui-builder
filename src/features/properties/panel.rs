use yew::prelude::*;
use crate::entities::cui_element::model::CuiElement;

#[derive(Properties, Clone)]
pub struct PropertiesPanelProps {
    pub selected_element: Option<CuiElement>,
    pub on_element_change: Callback<CuiElement>,
}

impl PartialEq for PropertiesPanelProps {
    fn eq(&self, other: &Self) -> bool {
        // Сравниваем только selected_element, так как Callback не реализует PartialEq
        self.selected_element == other.selected_element
    }
}

pub struct PropertiesPanel {
    props: PropertiesPanelProps,
}

impl Component for PropertiesPanel {
    type Message = ();
    type Properties = PropertiesPanelProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            props: ctx.props().clone() 
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <div class="properties-panel">
                {"Properties Panel"}
            </div>
        }
    }
} 