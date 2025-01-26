use window::{component::create_editors, window::create_window};

mod window;
mod components;

fn main() {
    let editors = create_editors();
    let app = create_window(editors);
    floem::launch(|| app);
}