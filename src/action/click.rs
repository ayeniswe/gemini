use std::{collections::HashMap, rc::Rc};

use dyn_clone::{clone_trait_object, DynClone};
use log::debug;
use winit::{
    event::{Event, MouseButton, WindowEvent},
    window::Window,
};

use crate::ui::{sync::Signal, widget::BaseWidget};

/// The `Click` struct allows widgets to have the ability
/// to respond to any mouse click event
#[derive(Clone)]
pub struct Click<State> {
    state: State,
    button_map: HashMap<MouseButton, Rc<dyn Fn(&mut State, &Window, &mut BaseWidget, Event<Signal>)>>,
}
impl<State> Click<State> {
    /// Create a new `Click` action
    ///
    /// The `state` provides the ability
    /// to react to the current state of any
    /// arbitrary instance
    pub fn new(state: State) -> Self {
        Self {
            state,
            button_map: HashMap::default(),
        }
    }
    /// Set a handler for a specific button type
    ///
    /// Types:
    /// - LeftButton
    /// - RightButton
    /// - MiddleButton
    /// - BackButton
    /// - ForwardButton
    pub fn on<F: Fn(&mut State, &Window, &mut BaseWidget, Event<Signal>) + Clone + 'static>(
        mut self,
        btn: MouseButton,
        callback: F,
    ) -> Self {
        self.button_map.insert(btn, Rc::new(callback));
        self
    }
}
impl<State: Clone> ClickHandler for Click<State> {
    fn apply(&mut self, window: &Window, widget: &mut BaseWidget, e: Event<Signal>) {
        match e {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CursorMoved { position, .. } => {
                    widget.state.hovered = widget.layout.is_inbounds(position.x, position.y);
                }
                WindowEvent::MouseInput { button, state, .. } => {
                    if widget.state.hovered && state.is_pressed() {
                        debug!("triggered {:?} button for widget: {}", button, widget.id);
                        let handler = self.button_map.get(&button);
                        if let Some(handler) = handler {
                            handler(&mut self.state, window, widget, e)
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}

/// The trait `ClickHandler` provides a
/// way for ergonomic use for
/// users to specify per button type
/// click actions
pub trait ClickHandler: DynClone {
    fn apply(&mut self, window: &Window, widget: &mut BaseWidget, e: Event<Signal>);
}
clone_trait_object!(ClickHandler);
