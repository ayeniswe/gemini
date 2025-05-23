//! A middleware system for handling dynamic UI behavior based on user actions.
//!
//! This module defines the `Action` enum and associated subactions that represent
//! discrete user-driven events or system-level interactions. These actions
//! are used to drive widget state updates dynamically, allowing the UI to
//! respond to events like hovering, clicking, or focusing in a structured
//! and extensible way.
//!

use std::rc::Rc;

use click::ClickHandler;
use hover::Hover;
use scroll::Scroll;
use winit::{dpi::PhysicalPosition, event::Event, window::Window};
use zoom::Zoom;

use crate::ui::{
    sync::{Signal, Trigger},
    widget::{container::Container, Widget, WidgetI},
};

pub mod click;
pub mod hover;
pub(crate) mod scroll;
pub mod zoom;

/// The `Action` enum acts as a middleware layer to dispatch event
/// to the appropiate handler based on action variants.
/// Each variants encapasulates its own logic on how to interpret the event
/// and makes the modification to the widgets
///
/// All actions can be stateful
#[derive(Clone)]
pub enum Action {
    /// Allows the user to alter the color
    /// upon hovering over this widget
    ///
    /// Similiar to `onhover` in javascript
    Hover(Hover),
    /// Allows the user to respond to clicks on the widget
    Click(Box<dyn ClickHandler>),
    /// Allows `Container` to be scrollable
    Scroll(Scroll),
    // Allows the user to zoom in and out of this widget
    // ZoomInOut(Zoom),
}
impl Action {
    pub(crate) fn apply_action(
        &mut self,
        trigger: Rc<Trigger>,
        widget: &Rc<dyn WidgetI>,
        event: Event<Signal>,
        cursor_pos: PhysicalPosition<f64>,
    ) {
        match self {
            Action::Hover(hover) => hover.apply(trigger, &mut widget.base_mut(), event),
            Action::Scroll(scroll) => scroll.apply(
                trigger,
                widget.as_any().downcast_ref::<Container>().unwrap(),
                event,
                cursor_pos,
            ),
            Action::Click(click) => click.apply(trigger, &mut widget.base_mut(), event),
            // _ => (),
        }
    }
}