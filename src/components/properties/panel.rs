use yew::prelude::*;
use web_sys::console;
use crate::models::Element;
use crate::core::component::Component;

fn find_element_by_id<'a>(elements: &'a [Element], id: &str) -> Option<&'a Element> {
    for element in elements {
        if element.id == id {
            return Some(element);
        }
        if let Some(found) = find_element_by_id(&element.children, id) {
            return Some(found);
        }
    }
    None
}

#[derive(Properties, PartialEq)]
pub struct PropertiesPanelProps {
    pub elements: Vec<Element>,
    pub selected_id: Option<String>,
    pub on_update_component: Callback<(String, Component)>,
}

#[function_component(PropertiesPanel)]
pub fn properties_panel(props: &PropertiesPanelProps) -> Html {
    let updating = use_state(|| false);
    let error_message = use_state(String::new);
    
    let on_update_wrapper = {
        let on_update_component = props.on_update_component.clone();
        let updating = updating.clone();
        let error_message = error_message.clone();
        
        Callback::from(move |(element_id, component): (String, Component)| {
            updating.set(true);
            error_message.set(String::new());
            
            console::log_1(&format!("Начало обновления компонента {} для элемента {}", 
                component.component_type(), element_id).into());
            
            on_update_component.emit((element_id, component));
            
            // В реальном приложении здесь должна быть проверка успешности обновления
            // Сейчас просто сбрасываем флаг через небольшую задержку
            let updating = updating.clone();
            let error_message = error_message.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(500).await;
                updating.set(false);
            });
        })
    };

    let selected_element = props.selected_id.as_ref().and_then(|id| {
        console::log_1(&format!("Поиск элемента с id: {}", id).into());
        let element = find_element_by_id(&props.elements, id);
        if let Some(e) = element {
            console::log_1(&format!("Найден элемент: {} с {} компонентами", 
                e.name, e.components.len()).into());
        }
        element
    });

    html! {
        <div class="properties-panel">
            {if let Some(element) = selected_element {
                html! {
                    <div class="properties-content">
                        <h3>{"Свойства"}</h3>
                        
                        if *updating {
                            <div class="update-indicator">
                                <span class="spinner"></span>
                                <span>{"Обновление..."}</span>
                            </div>
                        }
                        
                        if !(*error_message).is_empty() {
                            <div class="error-message">
                                {&*error_message}
                            </div>
                        }
                        
                        <div class="property-group">
                            <h4>{"Элемент"}</h4>
                            <div class="property-row">
                                <label>{"Имя"}</label>
                                <input type="text" value={element.name.clone()} />
                            </div>
                            <div class="property-row">
                                <label>{"Тип"}</label>
                                <span>{element.element_type.to_string()}</span>
                            </div>
                        </div>
                        {for element.components.iter().map(|component| {
                            let on_update = {
                                let on_update_wrapper = on_update_wrapper.clone();
                                let element_id = element.id.clone();
                                Callback::from(move |new_component: Component| {
                                    on_update_wrapper.emit((element_id.clone(), new_component));
                                })
                            };
                            component.render_properties_with_callback(on_update)
                        })}
                    </div>
                }
            } else {
                html! {
                    <div class="no-selection">
                        <p>{"Выберите элемент для просмотра его свойств"}</p>
                    </div>
                }
            }}
        </div>
    }
} 