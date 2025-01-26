use floem::{
    keyboard::{Key, NamedKey}, peniko::Color, reactive::SignalUpdate, views::{button, container, stack, Decorators}, IntoView, View
};
use crate::window::component::EditorComponents;

pub fn create_window(components: EditorComponents) -> impl IntoView {
    let EditorComponents {
        editor_a,
        editor_b,
        doc,
        hide_gutter_a,
        hide_gutter_b,
    } = components;

    let color = Color::rgb8(0x28, 0x2C, 0x34);

    let top_bar = container("Top Bar")
        .style(move |s| s.height(50.0).width_full().background(color));

    let left_sidebar = container("Left Sidebar")
        .style(move |s| s.width(100.0).height_full().background(color));

    let right_sidebar = container("Right Sidebar")
        .style(move |s| s.width(100.0).height_full().background(color));

    let main_content = stack((
        editor_a,
        editor_b,
        stack((
            button("Clear").action(move || {
                doc.edit_single(
                    floem::views::editor::core::selection::Selection::region(0, doc.text().len()),
                    "",
                    floem::views::editor::core::editor::EditType::DeleteSelection,
                );
            }),
            button("Flip Gutter").action(move || {
                hide_gutter_a.update(|hide| *hide = !*hide);
                hide_gutter_b.update(|hide| *hide = !*hide);
            }),
        ))
        .style(|s| s.width_full().flex_row().items_center().justify_center()),
    ))
    .style(|s| s.size_full().flex_col().items_center().justify_center());

    let layout = stack((
        top_bar,
        stack((
            left_sidebar,
            main_content.style(|s| s.flex()), 
            right_sidebar,
        ))
        .style(|s| s.flex_row().height_full()),
    ))
    .style(|s| s.flex_col().size_full());

    let id = layout.id();
    layout.on_key_up(
        Key::Named(NamedKey::F11),
        |m| m.is_empty(),
        move |_| id.inspect(),
    )
}
