use floem::{
    keyboard::{Key, NamedKey}, reactive::SignalUpdate, views::{button, stack, Decorators}, window::WindowId, IntoView, View
};
use crate::window::component::EditorComponents;
use crate::components::navbar::navbar;
use crate::components::left_panel::left_panel;
use crate::components::right_panel::right_panel;

pub fn create_window(components: EditorComponents, window_id: WindowId) -> impl IntoView {
    let EditorComponents {
        editor_a,
        editor_b,
        doc,
        hide_gutter_a,
        hide_gutter_b,
    } = components;

    let navbar = navbar(window_id);
    let left_sidebar = left_panel();
    let right_sidebar = right_panel();

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
        navbar,
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
