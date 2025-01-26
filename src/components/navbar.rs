use floem::{
    action, event::EventPropagation, peniko::Color, prelude::{create_rw_signal, RwSignal, SignalGet}, reactive::create_memo, taffy::JustifyContent, views::{button, container, drag_window_area, empty, label, stack, Decorators, Stack}, window::WindowId, View
};

fn right(window_maximized: RwSignal<bool>, window_id: WindowId) -> impl View {
    let background_color = Color::rgb8(0xFF, 0x00, 0x00);
    let text_color = Color::rgb8(0x7A, 0x7A, 0x7A);
    stack((
        drag_window_area(empty()).style(|s| s.height_pct(100.0).flex_basis(0.0).flex_grow(1.0)),
        stack((container(label(|| "1".to_string()).style(move |s| {
            s.font_size(10.0)
                .color(text_color)
                .border_radius(100.0)
                .margin_left(5.0)
                .margin_top(10.0)
                .background(background_color)
        }))
        .style(move |s| {
            s.absolute()
                .size_pct(100.0, 100.0)
                .justify_end()
                .items_end()
        }),))
        .style(move |s| s.margin_horiz(6.0)),
        window_controls_view(true, window_maximized, window_id),
    ))
    .style(|s| {
        s.flex_basis(0)
            .flex_grow(1.0)
            .justify_content(Some(JustifyContent::FlexEnd))
    })
    .debug_name("Right of top bar")
}

pub fn navbar(window_id: WindowId) -> Stack {
    let background_color = Color::rgb8(0x1F, 0x1F, 0x1F);
    let text_color = Color::rgb8(0x7A, 0x7A, 0x7A);
    let window_maximized = create_rw_signal(false);

    let navbar_title = label(|| "Navbar".to_string())
        .style(move |s| s.font_size(14.0).color(text_color).padding(10.0));

    stack((navbar_title, right(window_maximized, window_id)))
        .on_resize(move |rect| {
            let height = rect.height();
        })
        .style(move |s| {
            s.width_pct(100.0)
                .height(37.0)
                .items_center()
                .background(background_color)
                .border_bottom(1.0)
                .border_color(text_color)
        })
        .debug_name("Title / Top Bar")
}

pub fn window_controls_view(is_title: bool, window_maximized: RwSignal<bool>, window_id: WindowId) -> impl View {
    stack((
        button("Minimize".to_string())
            .style(|s| s.margin_right(16.0).margin_left(10.0))
            .on_click(move |_| {
                floem::action::minimize_window();
                EventPropagation::Continue
            }),

        button("Maximize".to_string())
            .style(|s| s.margin_right(16.0))
            .on_click({
                let window_maximized = window_maximized.clone();
                move |_| {
                    floem::action::set_window_maximized(
                        !window_maximized.get_untracked(),
                    );
                    EventPropagation::Continue
                }
            }),

        button("Close".to_string())
            .style(|s| s.margin_right(6.0))
            .on_click(move |_| {
                floem::close_window(window_id);
                EventPropagation::Continue
            }),
    ))
}