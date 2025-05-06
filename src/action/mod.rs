//! A middleware system for handling dynamic UI behavior based on user actions.
//!
//! This module defines the `Action` enum and associated subactions that represent
//! discrete user-driven events or system-level interactions. These actions
//! are used to drive widget state updates dynamically, allowing the UI to
//! respond to events like hovering, clicking, or focusing in a structured
//! and extensible way.

use hover::Hover;
use winit::{event::Event, window::Window};
use zoom::Zoom;

use crate::ui::widget::BaseWidget;

pub mod hover;
pub mod zoom;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Action {
    Hover(Hover),
    ZoomInOut(Zoom),
}

pub(crate) trait Actionable {
    fn apply_action(&mut self, window: &Window, widget: &mut BaseWidget, event: Event<()>);
}
