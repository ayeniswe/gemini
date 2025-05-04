//! The `button` module defines the [`Button`] widget.
//!
//! A `Button` is a rectangular interactive element with optional labeling and hover feedback.
//! It is designed to integrate with the `Widget` trait for unified UI behavior.
use crate::{impl_widget, ui::Widget};
use std::{cell::RefCell, rc::Rc};

use super::color::Color;
/// A clickable rectangular UI element that can display a label and reacts to hover events.
///
/// The `Button` widget supports hover color customization, positional layout,
/// and dimensions for rendering.
///
/// ## Aesthetics
/// Supports corner rounding
///
/// Example:
/// ```
/// let button = Button::new(100, 40);
/// button.borrow_mut().set_label("Click me");
/// ```
#[derive(Default, Debug, Clone)]
pub struct Button {
    pub label: Option<String>,
    pub hover_color: Option<Color>,
    pub color: Color,
    pub hovered: bool,
    pub pos: (u32, u32),
    pub width: u32,
    pub height: u32,
    pub radius: u32,
}
impl Button {
    pub fn new(width: u32, height: u32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            height,
            width,
            ..Default::default()
        }))
    }
}
impl_widget! {Button}
