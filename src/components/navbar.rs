use floem::{
    peniko::Color,
    views::{button, label, stack, Decorators, Stack}
};

pub fn navbar() -> Stack {
    let background_color = Color::rgb8(0x28, 0x2C, 0x34);
    let text_color = Color::rgb8(0xFF, 0xFF, 0xFF);

    let navbar_title =
        label(|| "Navbar".to_string()).style(move |s| s.font_size(14.0).color(text_color).padding(10.0));

    let home_button = button("Home")
        .action(|| println!("Home clicked"))
        .style(move |s| {
            s.margin(2.0)
                .padding(2.0)
                .background(Color::rgb8(0x3A, 0x40, 0x48))
                .color(text_color)
                .border_radius(2.0)
        });

    let settings_button = button("Settings")
        .action(|| println!("Settings clicked"))
        .style(move |s| {
            s.margin(2.0)
                .padding(2.0)
                .background(Color::rgb8(0x3A, 0x40, 0x48))
                .color(text_color)
                .border_radius(2.0)
        });

    let logout_button = button("Logout")
        .action(|| println!("Logout clicked"))
        .style(move |s| {
            s.margin(2.0)
                .padding(2.0)
                .background(Color::rgb8(0x3A, 0x40, 0x48))
                .color(text_color)
                .border_radius(2.0)
        });

    let navbar_buttons = stack((home_button, settings_button, logout_button))
        .style(|s| s.flex_row().items_center().justify_end());

    stack((navbar_title, navbar_buttons))
        .style(move |s| {
            s.height(20.0)
                .width_full()
                .flex_row()
                .items_center()
                .justify_between()
                .background(background_color)
        })
        .style(|s| s.height(20.0).width_full())
}
