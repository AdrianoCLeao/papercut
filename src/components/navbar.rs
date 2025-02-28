use floem::{
    action::{open_file, save_as},
    event::EventPropagation,
    file::{FileDialogOptions, FileInfo, FileSpec},
    peniko::Color,
    prelude::*,
    reactive::{create_effect, create_rw_signal, RwSignal, SignalGet, SignalUpdate},
    taffy::JustifyContent,
    views::{button, drag_window_area, empty, h_stack, stack, Decorators, Stack},
    window::WindowId,
    IntoView, View,
};

use dropdown::Dropdown;
use strum::IntoEnumIterator;

use crate::icons::svg;

fn right(window_maximized: RwSignal<bool>, window_id: WindowId) -> impl View {
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

pub fn window_controls_view(window_maximized: RwSignal<bool>, window_id: WindowId) -> impl View {
    stack((
        button(svg(svg::BACKWARD).class(ButtonClass))
            .style(|s| {
                s.margin_right(8.0)
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
        button(svg(svg::MAXIMIZE).class(ButtonClass))
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
                    floem::action::set_window_maximized(!window_maximized.get_untracked());
                    EventPropagation::Continue
                }
            }),
        button(svg(svg::CLOSE).class(ButtonClass))
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

#[derive(strum::EnumIter, Debug, PartialEq, Clone, Copy)]
enum FileAction {
    File,
    SelectFile,
    SelectFolder,
    SaveFile,
}

impl std::fmt::Display for FileAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

pub fn left() -> impl IntoView {
    let files = create_rw_signal("".to_string());
    let dropdown_selected_action = RwSignal::new(FileAction::File);

    create_effect(move |_| {
        match dropdown_selected_action.get() {
            FileAction::File => {
                println!("File");
            }
            FileAction::SelectFile => {
                open_file(
                    FileDialogOptions::new()
                        .force_starting_directory("/")
                        .title("Select file")
                        .allowed_types(vec![FileSpec {
                            name: "text",
                            extensions: &["txt", "rs", "md"],
                        }]),
                    move |file_info| {
                        if let Some(file) = file_info {
                            println!("Selected file: {:?}", file.path);
                            files.set(display_files(file));
                        }
                    },
                );
            }
            FileAction::SelectFolder => {
                open_file(
                    FileDialogOptions::new()
                        .select_directories()
                        .title("Select Folder"),
                    move |file_info| {
                        if let Some(file) = file_info {
                            println!("Selected folder: {:?}", file.path);
                            files.set(display_files(file));
                        }
                    },
                );
            }
            FileAction::SaveFile => {
                save_as(
                    FileDialogOptions::new()
                        .default_name("floem.file")
                        .title("Save file"),
                    move |file_info| {
                        if let Some(file) = file_info {
                            println!("Save file to: {:?}", file.path);
                            files.set(display_files(file));
                        }
                    },
                );
            }
        }
    });

    let dropdown = Dropdown::new_rw(dropdown_selected_action, FileAction::iter()) 
        .style(|s| {
            s.margin_right(8.0)
                .size(100.0, 30.0)
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
                .font_bold()
        });
        

    h_stack((dropdown,)).style(|s| s.justify_center())

}

fn display_files(file: FileInfo) -> String {
    let paths: Vec<&str> = file.path.iter().filter_map(|p| p.to_str()).collect();
    paths.join("\n")
}

pub fn navbar(window_id: WindowId) -> Stack {
    let background_color = Color::rgb8(0x1F, 0x1F, 0x1F);
    let text_color = Color::rgb8(0x7A, 0x7A, 0x7A);
    let window_maximized = create_rw_signal(false);

    let navbar_left = left();

    stack((navbar_left, right(window_maximized, window_id)))
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
