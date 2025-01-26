use floem::{peniko::Color, views::{container, Container}, views::Decorators};

pub fn right_panel() -> Container {
    let color = Color::rgb8(0x28, 0x2C, 0x34);
    
    let right_sidebar = container("right Sidebar")
        .style(move |s| s.width(100.0).height_full().background(color));

    return right_sidebar
}
