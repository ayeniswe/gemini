use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::{LogicalSize, PhysicalPosition},
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::{
    action::Actionable,
    render::{pixels_backend::PixelsRenderer, Renderer as _},
};

use super::widget::{canvas::Canvas, container::Container, Widget};

/// The main entry point for building and managing the UI tree.
///
/// The `DOM` struct is responsible for:
/// - Storing and updating widget state
/// - Handling input events (e.g., mouse movement)
/// - Triggering redraws and layout updates
pub struct DOM {
    nodes: Vec<Box<dyn Widget>>,
    renderer: PixelsRenderer,
    window: Window,
    event_loop: EventLoop<()>,
    cursor_position: PhysicalPosition<f64>,
}
impl DOM {
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
    /// Act on the widget apperance and behaviours based on the
    /// actions they subscribed to and only triggering action based
    /// on the actions logic
    fn apply_actions(window: &Window, node: &Box<dyn Widget>, event: Event<()>) {
        let mut actions = node.action_mut();
        let mut widget = node.base_mut();
        for action in actions.iter_mut() {
            action.apply_action(window, &mut widget, event.clone());
        }

        // Child nodes are possible and must invoke any events as well
        if let Some(canvas) = node.as_any().downcast_ref::<Canvas>() {
            // Handle all grid cells of canvas
            let grid = &*canvas.grid.borrow();
            if let Some(grid) = grid {
                grid.on_cell(|_, c| {
                    let mut actions = c.action_mut();
                    let mut widget = c.base_mut();
                    for action in actions.iter_mut() {
                        action.apply_action(window, &mut widget, event.clone());
                    }
                });
            }
        } else if let Some(container) = node.as_any().downcast_ref::<Container>() {
            for child in &container.children {
                DOM::apply_actions(window, child, event.clone());
            }
        }
    }
    pub fn run(mut self) {
        self.event_loop
            .run(|event, target| {
                // Handles core events that are always moinitored
                // for functionality
                match event {
                    Event::WindowEvent { ref event, .. } => match event {
                        // Updating and tracking cursor position
                        WindowEvent::CursorMoved { position, .. } => {
                            self.cursor_position = *position;
                        }
                        // Handle for closing window
                        WindowEvent::CloseRequested => target.exit(),
                        // Draw all nodes on the display
                        WindowEvent::RedrawRequested => {
                            self.renderer.clear();

                            for node in &self.nodes {
                                self.renderer.draw(node);
                            }

                            self.renderer.present();
                        }
                        _ => (),
                    },
                    _ => (),
                }
                
                for node in &self.nodes {
                    DOM::apply_actions(&self.window, node, event.clone());
                }
            })
            .unwrap();
    }
    pub fn add_widget<T: Widget + 'static>(&mut self, widget: T) {
        self.nodes.push(Box::new(widget));
    }
}
