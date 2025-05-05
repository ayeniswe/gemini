//! The `button` module defines the [`Button`] widget.
//!
//! A `Button` is a rectangular interactive element with optional labeling and hover feedback.
//! It is designed to integrate with the `Widget` trait for unified UI behavior.
use crate::{impl_widget, ui::Widget};

use super::{action::hover::Hover, color::Color, layout::Layout, style::Style, text::Text};

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
/// let button = button.set_label("Click me");
/// ```
#[derive(Default, Debug, Clone)]
pub struct Button {
    pub text: Text,
    pub hover: Hover,
    pub style: Style,
    pub layout: Layout,
}
impl Button {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            layout: Layout {
                w: width,
                h: height,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
impl_widget! {Button}
