use window::{component::{Component, LayoutDirection}, window::Window};

mod window;

fn main() {
    let mut window = Window::new("Main Window", 800, 600);

    let mut header = Component::new("Header", None, Some((800, 50)), Some((0, 0)));
    let footer = Component::new("Footer", None, Some((800, 50)), Some((0, 550)));
    let sidebar = Component::new("Sidebar", Some(LayoutDirection::Vertical), Some((200, 500)), Some((0, 50)));
    let mut content = Component::new("Content", Some(LayoutDirection::Horizontal), Some((600, 500)), Some((200, 50)));

    let button = Component::new("Button", None, Some((100, 50)), None);
    content.add_child(button);

    header.add_child(Component::new("Title", None, Some((800, 30)), None));
    window.add_component(header);
    window.add_component(sidebar);
    window.add_component(content);
    window.add_component(footer);

    window.run();
}