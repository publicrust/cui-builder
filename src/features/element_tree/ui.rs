use yew::prelude::*;
use crate::features::element_tree::model::{ElementTreeNode, ElementTreeProps};

#[function_component(ElementTree)]
pub fn element_tree(props: &ElementTreeProps) -> Html {
    let ElementTreeProps { root, on_select } = props.clone();

    html! {
        <div class="element-tree">
            <ElementTreeNodeView
                node={root}
                on_select={on_select}
            />
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct ElementTreeNodeViewProps {
    node: ElementTreeNode,
    on_select: Callback<String>,
}

#[function_component(ElementTreeNodeView)]
fn element_tree_node_view(props: &ElementTreeNodeViewProps) -> Html {
    let ElementTreeNodeViewProps { node, on_select } = props.clone();
    
    let onclick = {
        let id = node.id.clone();
        let on_select = on_select.clone();
        Callback::from(move |_| {
            on_select.emit(id.clone());
        })
    };

    html! {
        <div class="element-tree-node">
            <div class="element-tree-node-content" {onclick}>
                <span class="element-tree-node-name">{&node.name}</span>
            </div>
            if !node.children.is_empty() {
                <div class="element-tree-node-children">
                    { for node.children.iter().map(|child| {
                        html! {
                            <ElementTreeNodeView
                                node={child.clone()}
                                on_select={on_select.clone()}
                            />
                        }
                    })}
                </div>
            }
        </div>
    }
} 