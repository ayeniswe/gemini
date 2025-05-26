use std::{
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};
use winit::event_loop::EventLoopProxy;

use super::widget::WidgetI;

pub(crate) type UID = usize;

pub trait WidgetCallback: Fn(Rc<dyn WidgetI>) + Send + Sync + 'static {}
impl<F: Fn(Rc<dyn WidgetI>) + Send + Sync + 'static> WidgetCallback for F {}

/// `EventLoopProxy` user events
#[derive(Clone)]
pub enum Signal {
    /// Redraw widget
    Update(UID),
    /// Callback to apply changes to a widget
    /// before redrawing
    Callback((UID, Arc<dyn WidgetCallback>)),
}

/// The `Trigger` struct allows the user to trigger interactions
/// with the widgets on the UI main thread
#[derive(Clone)]
pub struct Trigger {
    proxy: Arc<Mutex<EventLoopProxy<Signal>>>,
    pub(crate) uid: UID,
}
impl Trigger {
    pub(crate) fn new(proxy: Arc<Mutex<EventLoopProxy<Signal>>>, uid: UID) -> Self {
        Self { proxy, uid }
    }
    /// Triggers update to widget
    pub fn update(&self) {
        let _ = self
            .proxy
            .lock()
            .unwrap()
            .send_event(Signal::Update(self.uid));
    }
    /// Triggers callback on widget before
    /// updating
    pub fn update_callback<F: WidgetCallback>(&self, callback: F) {
        let _ = self
            .proxy
            .lock()
            .unwrap()
            .send_event(Signal::Callback((self.uid, Arc::new(callback))));
    }
}

/// The `Thread` defines anything that has the ability
/// to run off the main thread
pub(crate) trait Thread {
    fn start(self: Arc<Self>, trigger: Trigger);
}

/// The `Emitter` trait allows user to customize
/// trigger actions to take place in a seperate thread
pub trait Emitter: Send + Sync + 'static {
    /// When the `Emitter` thread starts this `run` method gets called
    /// wrapped by its own thread
    fn run(self: Arc<Self>, trigger: Trigger);
}
impl<E: Emitter> Thread for E {
    fn start(self: Arc<Self>, trigger: Trigger) {
        let _ = thread::spawn(move || {
            self.run(trigger);
        });
    }
}
impl<E: Emitter> Thread for Arc<E> {
    fn start(self: Arc<Self>, trigger: Trigger) {
        let _ = thread::spawn(move || {
            <Arc<E> as Clone>::clone(&self).run(trigger);
        });
    }
}
