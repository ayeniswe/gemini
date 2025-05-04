//! The `tab` module defines the [`Tab`] widget.
//!
//! A `Tab` is a rectangular UI element that can be labeled,
//! positioned, styled with a hover color, and respond to mouse events.
//!
//! Tabs are meant to be used inside a container like [`UI`] to create
//! tabbed interfaces or navigation controls.
use std::{cell::RefCell, rc::Rc};

use crate::impl_widget;

use super::{color::Color, Widget};

/// A rectangular UI widget representing a single tab.
///
/// Each `Tab` has a size, optional label, position, and a hover color state.
/// You can configure a tab using the builder-style methods.
///
/// # Example
/// ```rust
/// let tab = Tab::new(100, 30)
/// tab.borrow_mut()
///    .set_label("Settings")
///    .set_pos(20, 10)
///    .on_hover(Color::RGB(200, 200, 255));
/// ```
#[derive(Default, Debug)]
pub struct Tab {
    pub label: Option<String>,
    pub hover_color: Option<Color>,
    pub color: Color,
    pub hovered: bool,
    pub pos: (u32, u32),
    pub width: u32,
    pub height: u32,
    pub radius: u32,
}
impl Tab {
    pub fn new(width: u32, height: u32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            height,
            width,
            ..Default::default()
        }))
    }
}
impl_widget! {Tab}
