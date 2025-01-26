use floem::{peniko::Color, views::{container, Container}, views::Decorators};

pub fn left_panel() -> Container {
    let color = Color::rgb8(0x28, 0x2C, 0x34);
    
    let left_sidebar = container("Left Sidebar")
        .style(move |s| s.width(100.0).height_full().background(color));

    return left_sidebar
}
