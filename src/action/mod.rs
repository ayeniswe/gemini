//! A middleware system for handling dynamic UI behavior based on user actions.
//!
//! This module defines the `Action` enum and associated subactions that represent
//! discrete user-driven events or system-level interactions. These actions
//! are used to drive widget state updates dynamically, allowing the UI to
//! respond to events like hovering, clicking, or focusing in a structured
//! and extensible way.
//!
use click::Click;
use hover::Hover;
use winit::{
    event::{Event, MouseButton},
    window::Window,
};
use zoom::Zoom;

use crate::ui::widget::BaseWidget;

pub mod click;
pub mod hover;
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
    /// Allows the user to zoom in and out of this widget
    ZoomInOut(Zoom),
    /// Allows the user to respond to clicks on the widget
    LeftClick(Click),
}
impl Actionable for Action {
    fn apply_action(&mut self, window: &Window, widget: &mut BaseWidget, event: Event<()>) {
        match self {
            Action::Hover(hover) => hover.apply(window, widget, event),
            Action::ZoomInOut(zoom) => zoom.apply(window, widget, event),
            Action::LeftClick(click) => click.apply(MouseButton::Left, window, widget, event),
        }
    }
}
pub(crate) trait Actionable {
    /// Decides the actions to apply to the widget base design
    ///
    /// This dispatches the event down to the appropriate action handler
    fn apply_action(&mut self, window: &Window, widget: &mut BaseWidget, event: Event<()>);
}
