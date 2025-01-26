use std::rc::Rc;

use floem::{
    keyboard::{Key, NamedKey}, prelude::SignalGet, reactive::{RwSignal, SignalUpdate}, views::{
        button,
        editor::{command::{Command, CommandExecuted}, core::{command::EditCommand, editor::EditType, selection::Selection}, text::{default_dark_color, Document, SimpleStyling}},
        stack, text_editor, Decorators, TextEditor,
    }, View
};

pub struct EditorComponents {
    pub editor_a: TextEditor,
    pub editor_b: TextEditor,
    pub doc: Rc<dyn Document>,
    pub hide_gutter_a: RwSignal<bool>,
    pub hide_gutter_b: RwSignal<bool>,
}

pub fn create_editors() -> EditorComponents {
    let text = std::env::args()
        .nth(1)
        .map(|s| std::fs::read_to_string(s).unwrap());
    let text = text.as_deref().unwrap_or("Hello world");

    let hide_gutter_a = RwSignal::new(false);
    let hide_gutter_b = RwSignal::new(true);

    let editor_a = text_editor(text)
        .styling(SimpleStyling::new())
        .style(|s| s.size_full())
        .editor_style(default_dark_color)
        .editor_style(move |s| s.hide_gutter(hide_gutter_a.get()));

    let editor_b = editor_a
        .shared_editor()
        .editor_style(default_dark_color)
        .editor_style(move |s| s.hide_gutter(hide_gutter_b.get()))
        .style(|s| s.size_full())
        .pre_command(|ev| {
            if matches!(ev.cmd, Command::Edit(EditCommand::Undo)) {
                println!("Undo command executed on editor B, ignoring!");
                return CommandExecuted::Yes;
            }
            CommandExecuted::No
        })
        .update(|_| {
            println!("Editor changed");
        })
        .placeholder("Some placeholder text");

    let doc = editor_a.doc();

    EditorComponents {
        editor_a,
        editor_b,
        doc,
        hide_gutter_a,
        hide_gutter_b,
    }
}
