#[derive(Debug, Clone)]
pub enum LayoutDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone)]
pub struct Component {
    pub name: String,
    pub children: Vec<Component>,
    pub layout: Option<LayoutDirection>,
    pub size: Option<(u32, u32)>, 
    pub position: Option<(i32, i32)>, 
}

impl Component {
    pub fn new(name: &str, layout: Option<LayoutDirection>, size: Option<(u32, u32)>, position: Option<(i32, i32)>) -> Self {
        Component {
            name: name.to_string(),
            children: Vec::new(),
            layout,
            size,
            position,
        }
    }

    pub fn add_child(&mut self, child: Component) {
        self.children.push(child);
    }
}