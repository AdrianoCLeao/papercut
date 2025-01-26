use floem::{kurbo::Size, window::WindowConfig, Application};
use window::{component::create_editors, window::create_window};

mod window;
mod components;

fn main() {
    let editors = create_editors();

    let app = Application::new().window(
        |_| create_window(editors), 
        Some(
            WindowConfig::default()
                .size(Size::new(800.0, 600.0)) 
                .undecorated(true)
        ),
    );

    app.run(); 
}