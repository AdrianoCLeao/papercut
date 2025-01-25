#[derive(Debug, Clone)]
enum LayoutDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone)]
struct Component {
    name: String,
    children: Vec<Component>,
    layout: Option<LayoutDirection>,
    size: Option<(u32, u32)>, 
    position: Option<(i32, i32)>, 
}

impl Component {
    fn new(name: &str, layout: Option<LayoutDirection>, size: Option<(u32, u32)>, position: Option<(i32, i32)>) -> Self {
        Component {
            name: name.to_string(),
            children: Vec::new(),
            layout,
            size,
            position,
        }
    }

    fn add_child(&mut self, child: Component) {
        self.children.push(child);
    }
}