use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;

#[derive(Clone, Debug, PartialEq)]
pub struct UnityTransform {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for UnityTransform {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        }
    }
}

impl UnityTransform {
    pub fn render_properties_with_callback(&self, on_update: Callback<super::Component>) -> Html {
        let x = self.x;
        let y = self.y;
        let width = self.width;
        let height = self.height;

        let on_x_change = {
            let mut transform = self.clone();
            let on_update = on_update.clone();
            Callback::from(move |e: Event| {
                let input: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
                transform.x = input.value().parse().unwrap_or(x);
                on_update.emit(super::Component::UnityTransform(transform.clone()));
            })
        };

        let on_y_change = {
            let mut transform = self.clone();
            let on_update = on_update.clone();
            Callback::from(move |e: Event| {
                let input: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
                transform.y = input.value().parse().unwrap_or(y);
                on_update.emit(super::Component::UnityTransform(transform.clone()));
            })
        };

        let on_width_change = {
            let mut transform = self.clone();
            let on_update = on_update.clone();
            Callback::from(move |e: Event| {
                let input: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
                transform.width = input.value().parse().unwrap_or(width);
                on_update.emit(super::Component::UnityTransform(transform.clone()));
            })
        };

        let on_height_change = {
            let mut transform = self.clone();
            let on_update = on_update;
            Callback::from(move |e: Event| {
                let input: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
                transform.height = input.value().parse().unwrap_or(height);
                on_update.emit(super::Component::UnityTransform(transform.clone()));
            })
        };

        html! {
            <div class="component-properties">
                <div class="property">
                    <label>{"X:"}</label>
                    <input type="number" value={x.to_string()} onchange={on_x_change}/>
                </div>
                <div class="property">
                    <label>{"Y:"}</label>
                    <input type="number" value={y.to_string()} onchange={on_y_change}/>
                </div>
                <div class="property">
                    <label>{"Width:"}</label>
                    <input type="number" value={width.to_string()} onchange={on_width_change}/>
                </div>
                <div class="property">
                    <label>{"Height:"}</label>
                    <input type="number" value={height.to_string()} onchange={on_height_change}/>
                </div>
            </div>
        }
    }
} 