use std::{
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
};

use pixels::{Pixels, SurfaceTexture};
use rand::Rng;
use winit::{
    dpi::{LogicalSize, PhysicalPosition},
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopBuilder, EventLoopProxy},
    window::{Window, WindowBuilder},
};

use crate::{
    action::Actionable,
    render::{pixels_backend::PixelsRenderer, pre::PreRenderer, Renderer as _},
};

use super::{
    sync::{Signal, Trigger},
    widget::{canvas::Canvas, container::Container, Widget},
};

/// The main entry point for building and managing the UI tree.
///
/// The `DOM` struct is responsible for:
/// - Storing and updating widget state
/// - Handling input events (e.g., mouse movement)
/// - Triggering redraws and layout updates
pub struct DOM {
    nodes: Vec<Rc<dyn Widget>>,
    renderer: PixelsRenderer,
    pre_renderer: PreRenderer,
    window: Window,
    event_loop: EventLoop<Signal>,
    proxy: Arc<Mutex<EventLoopProxy<Signal>>>,
    cursor_position: PhysicalPosition<f64>,
    signals_route: HashMap<u64, Rc<dyn Widget>>,
}
impl DOM {
    pub fn new(width: u32, height: u32) -> Self {
        let event_loop = EventLoopBuilder::<Signal>::with_user_event()
            .build()
            .unwrap();

        // Allow other threads to send info to
        // main UI thread
        let proxy = event_loop.create_proxy();

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
            pre_renderer: PreRenderer::new(),
            renderer: PixelsRenderer::new(pixels),
            window,
            nodes: Vec::default(),
            event_loop,
            proxy: Arc::new(Mutex::new(proxy)),
            cursor_position: PhysicalPosition::default(),
            signals_route: HashMap::default(),
        }
    }
    /// Act on the widget apperance and behaviours based on the
    /// actions they subscribed to and only triggering action based
    /// on the actions logic
    fn apply_actions(
        window: &Window,
        node: &Rc<dyn Widget>,
        event: Event<Signal>,
        cursor_pos: PhysicalPosition<f64>,
    ) {
        let mut actions = node.action_mut();
        for action in actions.iter_mut() {
            action.apply_action(window, node, event.clone(), cursor_pos);
        }

        // Child nodes are possible and must invoke any events as well
        if let Some(canvas) = node.as_any().downcast_ref::<Canvas>() {
            // Handle all grid cells of canvas
            let grid = &*canvas.grid.borrow();
            if let Some(grid) = grid {
                grid.on_cell(|_, cell| {
                    let mut actions = cell.action_mut();
                    let cell: Rc<dyn Widget> = cell.clone();
                    for action in actions.iter_mut() {
                        action.apply_action(window, &cell, event.clone(), cursor_pos);
                    }
                });
            }
        } else if let Some(container) = node.as_any().downcast_ref::<Container>() {
            for child in &container.children {
                DOM::apply_actions(window, child, event.clone(), cursor_pos);
            }
        }
    }
    /// Widgets may need ui changes off thread
    /// emitters allow changes to be processed in a queue
    /// style using `Signal`s
    fn apply_emitters(&mut self, widget: &Rc<dyn Widget>) {
        // Some widget may be connected to user thread
        // We need a unique mapping for event signal routing
        if let Some(emit) = widget.emitter().cloned() {
            let uid: u64 = rand::thread_rng().gen();
            self.signals_route.insert(uid, widget.clone());
            emit.start(Trigger::new(self.proxy.clone(), uid));
        }

        if let Some(container) = widget.as_any().downcast_ref::<Container>() {
            for child in &container.children {
                self.apply_emitters(child);
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
                                self.pre_renderer.adjust(node);

                                self.renderer.draw(node);
                            }

                            self.renderer.present();
                        }
                        _ => (),
                    },
                    Event::UserEvent(ref signal) => match signal {
                        Signal::Update(data) => {
                            // We need to route the signals in a way to denote what
                            // widget to target
                            let (id, update) = data;
                            let widget = self.signals_route.get(&id).unwrap();

                            // Apply changes on main thread
                            update(&mut widget.base_mut());

                            // Costly if no changes occured but
                            // this is left up to the user
                            // care on performance
                            self.window.request_redraw();
                        }
                    },
                    _ => (),
                }

                for node in &self.nodes {
                    DOM::apply_actions(&self.window, node, event.clone(), self.cursor_position);
                }
            })
            .unwrap();
    }
    pub fn add_widget<T: Widget + 'static>(&mut self, widget: T) {
        let widget: Rc<dyn Widget> = Rc::new(widget);
        let widget_clone = widget.clone();
        self.nodes.push(widget);

        // Auto-start any emitters for widgets
        self.apply_emitters(&widget_clone);
    }
}
