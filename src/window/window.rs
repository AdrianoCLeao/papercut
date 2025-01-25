use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};
use crate::window::component::{Component, LayoutDirection};

pub struct Window {
    root: Component,
    event_loop: EventLoop<()>,
    winit_window: winit::window::Window,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new();
        let winit_window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            .build(&event_loop)
            .expect("Failed to create window");

        let root_component = Component::new(
            title,
            Some(LayoutDirection::Vertical),
            Some((width, height)),
            Some((0, 0)),
        );

        Window {
            root: root_component,
            event_loop,
            winit_window,
        }
    }

    pub fn add_component(&mut self, component: Component) {
        self.root.add_child(component);
    }

    pub fn get_root(&self) -> &Component {
        &self.root
    }

    pub fn run(self) {
        let mut root = self.root;
        let mut event_loop = self.event_loop;
        let winit_window = self.winit_window;
    
        event_loop.run_return(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
    
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("Window close requested, exiting...");
                    *control_flow = ControlFlow::Exit;
                }
                Event::MainEventsCleared => {
                    render_component(&root, 0);
                }
                _ => (),
            }
        });
    }

    fn render_component(&self, component: &Component, depth: usize) {
        let indent = "  ".repeat(depth);
        println!("{}Component: {}", indent, component.name);

        for child in &component.children {
            self.render_component(child, depth + 1);
        }
    }
}

fn render_component(component: &Component, depth: usize) {
    let indent = "  ".repeat(depth);
    println!("{}Component: {}", indent, component.name);

    for child in &component.children {
        render_component(child, depth + 1);
    }
}