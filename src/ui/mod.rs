//! The top-level UI module for building and organizing user interface
//! components.
//!
//! This module serves as the entry point for all UI-related functionality,
//! including widget definitions, layout systems, styling, and user
//! interaction handling. It is designed to be modular and extensible,
//! supporting a wide range of interactive graphical applications.
//!
//! The `ui` module is intended to be the main hub for all user
//! interface logic, serving as a foundation for complex frontends,
//! editors, or graphical tools.

use color::Color;
use pixels::{Pixels, SurfaceTexture};
use widget::Widget;
use winit::{
    dpi::{LogicalSize, PhysicalPosition},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{
    action::{self, Action, Actionable},
    render::{pixels_backend::PixelsRenderer, Renderer as _},
};

pub mod color;
pub mod layout;
pub mod style;
pub mod text;
pub mod widget;

/// The main entry point for building and managing the UI tree.
///
/// The `DOM` struct is responsible for:
/// - Storing and updating widget state
/// - Handling input events (e.g., mouse movement)
/// - Triggering redraws and layout updates
pub struct DOM<T: Widget> {
    nodes: Vec<T>,
    renderer: PixelsRenderer,
    window: Window,
    event_loop: EventLoop<()>,
    cursor_position: PhysicalPosition<f64>,
}
impl<T: Widget> DOM<T> {
    pub fn new(width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new().unwrap();

        // Window to contain the application
        let window = WindowBuilder::new()
            .with_title("Gemini - UI Framework")
            .with_inner_size(LogicalSize::new(width, height))
            .build(&event_loop)
            .unwrap();

        // Backend to render ui drawings
        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
        let pixels = Pixels::new(size.width, size.height, surface_texture).unwrap();

        Self {
            renderer: PixelsRenderer::new(pixels),
            window,
            nodes: Vec::default(),
            event_loop,
            cursor_position: PhysicalPosition::default(),
        }
    }
    pub fn run(mut self) {
        self.event_loop
            .run(|event, target| {
                match event {
                    // Event::NewEvents(start_cause) => todo!(),
                    Event::WindowEvent { ref event, .. } => match event {
                        WindowEvent::CursorMoved { position, .. } => {
                            self.cursor_position = *position;
                        }
                        WindowEvent::CloseRequested => target.exit(),
                        WindowEvent::RedrawRequested => {
                            self.renderer.clear();

                            for node in &self.nodes {
                                self.renderer.draw_widget(node);
                            }

                            self.renderer.present();
                        }
                        _ => (),
                    },

                    _ => (),
                }
                for node in &mut self.nodes {
                    let mut actions = node.action_mut();
                    let mut widget = node.base_mut();
                    dbg!(widget.layout.w);
                    for action in actions.iter_mut() {
                       action.apply_action(&self.window, &mut widget, event.clone());
                    }
                }
            })
            .unwrap();
    }
    pub fn add_widget(&mut self, widget: T) {
        self.nodes.push(widget);
    }
}
