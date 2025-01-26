use floem::{
    keyboard::{Key, NamedKey},
    reactive::{RwSignal, SignalGet, SignalUpdate},
    views::{
        button,
        editor::{
            command::{Command, CommandExecuted},
            core::{command::EditCommand, editor::EditType, selection::Selection},
            text::{default_dark_color, SimpleStyling},
        },
        stack, text_editor, Decorators,
    },
    IntoView, View,
};

use crate::window::component::EditorComponents;

pub fn create_window(components: EditorComponents) -> impl IntoView{
    let EditorComponents {
        editor_a,
        editor_b,
        doc,
        hide_gutter_a,
        hide_gutter_b,
    } = components;

    let view = stack((
        editor_a,
        editor_b,
        stack((
            button("Clear").action(move || {
                doc.edit_single(
                    Selection::region(0, doc.text().len()),
                    "",
                    EditType::DeleteSelection,
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

    let id = view.id();
    view.on_key_up(
        Key::Named(NamedKey::F11),
        |m| m.is_empty(),
        move |_| id.inspect(),
    )
}
