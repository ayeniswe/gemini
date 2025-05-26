use log::debug;
use std::{collections::HashMap, rc::Rc};
use winit::event::{ElementState, Event, WindowEvent};

use crate::ui::{
    sync::{Signal, Trigger},
    widget::BaseWidget,
};

use super::ActionHandler;

/// The `MouseButton` struct are different
/// types of mouse buttons and button states
///
/// The `Release` variant represents
/// the state of a button being released
/// the default variant is the button
/// being pressed
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MouseButton {
    LeftButtonRelease,
    LeftButton,
    RightButtonRelease,
    RightButton,
    MiddleButtonRelease,
    MiddleButton,
    ForwardButtonRelease,
    ForwardButton,
    BackButtonRelease,
    BackButton,
    OtherButtonReleased(u16),
    OtherButton(u16),
}
/// The `Click` struct allows widgets to have the ability
/// to respond to any mouse click event
#[derive(Clone)]
pub struct Click<State> {
    state: State,
    button_map:
        HashMap<MouseButton, Rc<dyn Fn(&mut State, Rc<Trigger>, &mut BaseWidget, Event<Signal>)>>,
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
    pub fn on<F: Fn(&mut State, Rc<Trigger>, &mut BaseWidget, Event<Signal>) + Clone + 'static>(
        mut self,
        btn: MouseButton,
        callback: F,
    ) -> Self {
        self.button_map.insert(btn, Rc::new(callback));
        self
    }
}
impl<State: Clone> ActionHandler for Click<State> {
    fn apply(&mut self, trigger: Rc<Trigger>, widget: &mut BaseWidget, e: Event<Signal>) {
        match e {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CursorMoved { position, .. } => {
                    widget.state.hovered = widget.layout.is_inbounds(position.x, position.y);
                }
                WindowEvent::MouseInput { button, state, .. } => {
                    let button = match (button, state) {
                        (winit::event::MouseButton::Left, ElementState::Pressed) => {
                            MouseButton::LeftButton
                        }
                        (winit::event::MouseButton::Left, ElementState::Released) => {
                            MouseButton::LeftButtonRelease
                        }
                        (winit::event::MouseButton::Right, ElementState::Pressed) => {
                            MouseButton::RightButton
                        }
                        (winit::event::MouseButton::Right, ElementState::Released) => {
                            MouseButton::RightButtonRelease
                        }
                        (winit::event::MouseButton::Middle, ElementState::Pressed) => {
                            MouseButton::MiddleButton
                        }
                        (winit::event::MouseButton::Middle, ElementState::Released) => {
                            MouseButton::MiddleButtonRelease
                        }
                        (winit::event::MouseButton::Back, ElementState::Pressed) => {
                            MouseButton::BackButton
                        }
                        (winit::event::MouseButton::Back, ElementState::Released) => {
                            MouseButton::BackButtonRelease
                        }
                        (winit::event::MouseButton::Forward, ElementState::Pressed) => {
                            MouseButton::ForwardButton
                        }
                        (winit::event::MouseButton::Forward, ElementState::Released) => {
                            MouseButton::ForwardButtonRelease
                        }
                        (winit::event::MouseButton::Other(v), ElementState::Pressed) => {
                            MouseButton::OtherButton(*v)
                        }
                        (winit::event::MouseButton::Other(v), ElementState::Released) => {
                            MouseButton::OtherButtonReleased(*v)
                        }
                    };

                    if widget.state.hovered {
                        let handler = self.button_map.get(&button);
                        if let Some(handler) = handler {
                            debug!("triggered {:?} for widget: {}", button, widget.id);
                            handler(&mut self.state, trigger, widget, e)
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}
