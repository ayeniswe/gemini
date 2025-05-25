use std::{
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
};

use log::debug;
use pixels::{Pixels, SurfaceTexture};
use rand::Rng as _;
use winit::{
    dpi::{LogicalSize, PhysicalPosition},
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopBuilder, EventLoopProxy},
    window::{Window, WindowBuilder},
};

use crate::render::{pixels_backend::PixelsRenderer, pre::PreRenderer, Renderer};

use super::{
    sync::{Signal, Trigger, UID},
    widget::{canvas::Canvas, container::Container, Widget, WidgetI},
};

/// The main entry point for building and managing the UI tree.
///
/// The `DOM` struct is responsible for:
/// - Storing and updating widget state
/// - Handling input events (e.g., mouse movement)
/// - Triggering redraws and layout updates
pub struct DOM {
    renderer: PixelsRenderer,
    pre_renderer: PreRenderer,
    window: Window,
    event_loop: EventLoop<Signal>,
    proxy: Arc<Mutex<EventLoopProxy<Signal>>>,
    cursor_position: PhysicalPosition<f64>,
    nodes: Vec<Rc<dyn WidgetI>>,
    nodes_ref: HashMap<usize, Rc<dyn WidgetI>>,
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
            nodes_ref: HashMap::default(),
        }
    }
    /// Act on the widget apperance and behaviours based on the
    /// actions they subscribed to and only triggering action based
    /// on the actions logic
    fn apply_actions(
        node: &Rc<dyn WidgetI>,
        event: Event<Signal>,
        cursor_pos: PhysicalPosition<f64>,
    ) {
        let mut actions = node.action_mut();
        for action in actions.iter_mut() {
            action.apply_action(node.trigger(), node, event.clone(), cursor_pos);
        }

        // Child nodes are possible and must invoke any events as well
        if let Some(canvas) = node.as_any().downcast_ref::<Canvas>() {
            // Handle all grid cells of canvas
            let grid = &*canvas.grid.borrow();
            if let Some(grid) = grid {
                grid.on_cell(|_, cell| {
                    let mut actions = cell.action_mut();
                    let cell: Rc<dyn WidgetI> = cell.clone();
                    for action in actions.iter_mut() {
                        action.apply_action(cell.trigger(), &cell, event.clone(), cursor_pos);
                    }
                });
            }
        } else if let Some(container) = node.as_any().downcast_ref::<Container>() {
            for child in &container.children {
                DOM::apply_actions(child, event.clone(), cursor_pos);
            }
        }
    }
    /// Widgets may need ui changes off thread
    /// emitters allow changes to be processed in a queue
    /// style using `Signal`s
    fn apply_emitters(&mut self, widget: &Rc<dyn WidgetI>) {
        // Some widget may be connected to user thread
        // We need a unique mapping for event signal routing
        if let Some(emit) = widget.emitter().cloned() {
            emit.start(Trigger::new(self.proxy.clone(), widget.trigger().uid));
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
                        Signal::Update(id) => {
                            // We need to route the signals in a way to denote what
                            // widget to target
                            let widget = self.nodes_ref.get(id).unwrap();

                            // To save on performance we only need to clean whats
                            // targeted
                            let (x, y, h, w) = widget.base().layout.into();
                            self.renderer.dirty_clear(x, y, h, w);

                            self.renderer.draw(widget);

                            self.renderer.present();

                            debug!("redrawing widget: {}", &widget.base().id);
                        }
                    },
                    _ => (),
                }

                for node in &self.nodes {
                    DOM::apply_actions(node, event.clone(), self.cursor_position);
                }
            })
            .unwrap();
    }
    fn add_widgets(&mut self, widget: Rc<dyn WidgetI>) {
        // Attach trigger to allow user to trigger redraws on this widget
        // later
        let uid: UID = rand::thread_rng().gen();
        *widget.internal_trigger_mut() = Some(Rc::new(Trigger::new(self.proxy.clone(), uid)));

        self.nodes_ref.insert(uid, widget.clone());
        self.nodes.push(widget.clone());

        if let Some(canvas) = widget.as_any().downcast_ref::<Canvas>() {
            // Handle all grid cells of canvas
            let grid = &*canvas.grid.borrow();
            if let Some(grid) = grid {
                grid.on_cell(|_, cell| {
                    let cell: Rc<dyn WidgetI> = cell.clone();
                    let uid: UID = rand::thread_rng().gen();
                    *cell.internal_trigger_mut() =
                        Some(Rc::new(Trigger::new(self.proxy.clone(), uid)));
                    self.nodes_ref.insert(uid, cell);
                });
            }
        } else if let Some(container) = widget.as_any().downcast_ref::<Container>() {
            for child in &container.children {
                self.add_widgets(child.clone());
            }
        }

        // Auto-start any emitters for widgets
        self.apply_emitters(&widget);
    }
    pub fn add_widget<T: WidgetI + 'static>(&mut self, widget: T) {
        self.add_widgets(Rc::new(widget));
    }
}
