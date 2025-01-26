use floem::{
    event::EventPropagation, peniko::Color, prelude::{create_rw_signal, RwSignal, SignalGet}, taffy::JustifyContent, views::{button, container, drag_window_area, empty, label, stack, Decorators, Stack}, window::WindowId, View
};

fn right(window_maximized: RwSignal<bool>, window_id: WindowId) -> impl View {
    let background_color = Color::rgb8(0xFF, 0x00, 0x00);
    let text_color = Color::rgb8(0x7A, 0x7A, 0x7A);
    stack((
        drag_window_area(empty()).style(|s| s.height_pct(100.0).flex_basis(0.0).flex_grow(1.0)),
        window_controls_view(window_maximized, window_id),
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

pub fn window_controls_view(window_maximized: RwSignal<bool>, window_id: WindowId) -> impl View {
    stack((
        button("─".to_string())
        .style(|s| {
            s.margin_right(8.0)
                .size(30.0, 30.0)
                .justify_center() 
                .items_center()
                .background(Color::rgb8(31, 31, 31)) 
                .color(Color::rgb8(122, 122, 122))
                .hover(|s| {
                    s.background(Color::rgb8(43, 43, 43))
                        .color(Color::rgb8(122, 122, 122))
                        .border_color(Color::rgb8(43, 43, 43)) 
                })
                .border_color(Color::rgb8(31, 31, 31))
                .font_bold()
                .border_radius(0)
                .padding(1.0)
            })
            .on_click(move |_| {
                floem::action::minimize_window();
                EventPropagation::Continue
            }),

        button("🗗".to_string())
        .style(|s| {
            s.margin_right(8.0)
                .size(30.0, 30.0)
                .justify_center() 
                .items_center()
                .background(Color::rgb8(31, 31, 31)) 
                .color(Color::rgb8(122, 122, 122))
                .hover(|s| {
                    s.background(Color::rgb8(43, 43, 43))
                        .color(Color::rgb8(122, 122, 122))
                        .border_color(Color::rgb8(43, 43, 43)) 
                })
                .border_color(Color::rgb8(31, 31, 31))
                .border_radius(0)
                .padding(1.0)
                .padding_horiz(4.0)
                .padding_bottom_pct(4.0)
                .font_size(14.0)
            })
            .on_click({
                let window_maximized = window_maximized.clone();
                move |_| {
                    floem::action::set_window_maximized(
                        !window_maximized.get_untracked(),
                    );
                    EventPropagation::Continue
                }
            }),

        button("X".to_string())
        .style(|s| {
            s.margin_right(8.0)
                .size(30.0, 30.0)
                .justify_center() 
                .items_center()
                .background(Color::rgb8(31, 31, 31)) 
                .color(Color::rgb8(122, 122, 122))
                .hover(|s| {
                    s.background(Color::rgb8(43, 43, 43))
                        .color(Color::rgb8(122, 122, 122))
                        .border_color(Color::rgb8(43, 43, 43)) 
                })
                .border_color(Color::rgb8(31, 31, 31))
                .border_radius(0)
                .padding(1.0)
                .padding_horiz(4.0)
                .font_size(14.0)
                .font_bold()
            })
            .on_click(move |_| {
                floem::close_window(window_id);
                EventPropagation::Continue
            }),
    ))
}