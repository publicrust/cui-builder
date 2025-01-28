use crate::shared::lib::component::Component;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ElementTreeNode {
    pub id: String,
    pub name: String,
    pub component: Component,
    pub children: Vec<ElementTreeNode>,
    pub is_expanded: bool,
}

impl ElementTreeNode {
    pub fn new(id: String, name: String, component: Component) -> Self {
        Self {
            id,
            name,
            component,
            children: Vec::new(),
            is_expanded: false,
        }
    }

    pub fn add_child(&mut self, child: ElementTreeNode) {
        self.children.push(child);
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct ElementTreeProps {
    pub root: ElementTreeNode,
    pub on_select: Callback<String>,
} 