use crate::window::component::{Component, LayoutDirection};

pub struct Window {
    root: Component,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let root_component = Component::new(
            title,
            Some(LayoutDirection::Vertical),
            Some((width, height)),
            Some((0, 0)),
        );
        Window {
            root: root_component,
        }
    }

    pub fn add_component(&mut self, component: Component) {
        self.root.add_child(component);
    }

    pub fn get_root(&self) -> &Component {
        &self.root
    }

    pub fn render(&self) {
        println!("Rendering window: {:?}", self.root);
        self.render_component(&self.root, 0);
    }

    fn render_component(&self, component: &Component, depth: usize) {
        let indent = "  ".repeat(depth);
        println!("{}Component: {}", indent, component.name);

        for child in &component.children {
            self.render_component(child, depth + 1);
        }
    }
}
