use std::{
    ops::Deref,
    sync::{Arc, Mutex},
    thread,
};
use winit::event_loop::EventLoopProxy;

use super::widget::BaseWidget;

type UID = u64;

pub trait WidgetCallback: Fn(&mut BaseWidget) + Send + Sync + 'static {}
impl<F: Fn(&mut BaseWidget) + Send + Sync + 'static> WidgetCallback for F {}

/// `EventLoopProxy` user events
#[derive(Clone)]
pub enum Signal {
    /// Callback to apply changes to a widget
    Update((UID, Arc<dyn WidgetCallback>)),
}

/// The `Trigger` struct allows the user to trigger interactions
/// with the widgets on the UI main thread
pub struct Trigger {
    proxy: Arc<Mutex<EventLoopProxy<Signal>>>,
    uid: UID,
}
impl Trigger {
    pub(crate) fn new(proxy: Arc<Mutex<EventLoopProxy<Signal>>>, uid: UID) -> Self {
        Self { proxy, uid }
    }
    /// Triggers update to widget connection
    pub fn trigger_update<F: WidgetCallback>(&self, callback: F) {
        let _ = self
            .proxy
            .lock()
            .unwrap()
            .send_event(Signal::Update((self.uid, Arc::new(callback))));
    }
}

/// The `Thread` defines anything that has the ability
/// to run off the main thread
pub(crate) trait Thread {
    fn start(self: Arc<Self>, trigger: Trigger);
}

/// The `Emitter` trait allows user to customize
/// trigger actions to take place when a widget
/// gets a `Signal`
pub trait Emitter: Send + Sync + 'static {
    /// When the `Emitter` thread starts this `run` method gets called
    /// wrapped by its own thread
    fn run(&self, trigger: Trigger);
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
            self.run(trigger);
        });
    }
}
