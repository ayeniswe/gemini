use log::debug;
use std::rc::Rc;
use winit::event::{Event, WindowEvent};

use crate::ui::{
    sync::{Signal, Trigger},
    widget::BaseWidget,
};

use super::ActionHandler;

/// The `CursorMove` struct allows widgets to have the ability
/// to respond to any mouse move event
#[derive(Clone)]
pub struct CursorMove<State> {
    state: State,
    handler: Rc<dyn Fn(&mut State, Rc<Trigger>, &mut BaseWidget, Event<Signal>)>,
}
impl<State> CursorMove<State> {
    /// Create a new `CursorMove` action
    ///
    /// The `state` provides the ability
    /// to react to the current state of any
    /// arbitrary instance
    pub fn new<F: Fn(&mut State, Rc<Trigger>, &mut BaseWidget, Event<Signal>) + Clone + 'static>(
        state: State,
        callback: F,
    ) -> Self {
        Self {
            state,
            handler: Rc::new(callback),
        }
    }
}
impl<State: Clone> ActionHandler for CursorMove<State> {
    fn apply(&mut self, trigger: Rc<Trigger>, widget: &mut BaseWidget, e: Event<Signal>) {
        match e {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CursorMoved { position, .. } => {
                    widget.state.hovered = widget.layout.is_inbounds(position.x, position.y);

                    if widget.state.hovered {
                        debug!("triggered on cursor move for widget: {}", widget.id);
                        let handler = &self.handler;
                        handler(&mut self.state, trigger, widget, e)
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}
