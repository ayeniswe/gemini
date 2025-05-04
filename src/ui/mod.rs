//! The `ui` module is the root of the UI framework.
//!
//! This module coordinates layout, event handling, and rendering
//! of all UI components such as tabs, buttons, and panels.
//!
//! Use the [`UI`] struct to construct and manage a full user interface.
//! It acts as the top-level container that holds and updates all widgets.

use color::Color;

pub mod button;
pub mod color;
pub mod tab;

/// The main entry point for building and managing the UI tree.
///
/// The `UI` struct is responsible for:
/// - Storing and updating widget state
/// - Handling input events (e.g., mouse movement)
/// - Triggering redraws and layout updates
///
/// Example usage:
/// ```rust
/// let mut ui = UI::new();
/// // Add tabs, handle events, render, etc.
/// ```
pub struct UI {}

pub trait Widget {
    fn pos(&self) -> (u32, u32);
    fn pos_mut(&mut self) -> &mut (u32, u32);
    fn height(&self) -> u32;
    fn height_mut(&mut self) -> &mut u32;
    fn width(&self) -> u32;
    fn width_mut(&mut self) -> &mut u32;
    fn radius(&self) -> u32;
    fn radius_mut(&mut self) -> &mut u32;
    fn label(&self) -> &Option<String>;
    fn label_mut(&mut self) -> &mut Option<String>;
    fn hover_color(&self) -> &Option<Color>;
    fn hover_color_mut(&mut self) -> &mut Option<Color>;
    fn color(&self) -> &Color;
    fn color_mut(&mut self) -> &mut Color;
    fn hovered(&self) -> bool;
    fn hovered_mut(&mut self) -> &mut bool;
    fn set_label(&mut self, label: &str) -> &mut Self {
        *self.label_mut() = Some(label.into());
        self
    }
    fn set_x(&mut self, x: u32) -> &mut Self {
        *self.pos_mut() = (x, self.pos().1);
        self
    }
    fn set_y(&mut self, y: u32) -> &mut Self {
        *self.pos_mut() = (self.pos().0, y);
        self
    }
    fn set_height(&mut self, height: u32) -> &mut Self {
        *self.height_mut() = height;
        self
    }
    fn set_width(&mut self, width: u32) -> &mut Self {
        *self.width_mut() = width;
        self
    }
    fn set_radius(&mut self, radius: u32) -> &mut Self {
        *self.radius_mut() = radius;
        self
    }
    fn set_hover_color(&mut self, color: Color) -> &mut Self {
        *self.hover_color_mut() = Some(color);
        self
    }
    fn set_color(&mut self, color: Color) -> &mut Self {
        *self.color_mut() = color;
        self
    }
    fn update_hover_state(&mut self, mx: u32, my: u32) {
        let (x, y) = self.pos();
        *self.hovered_mut() =
            mx >= x && mx <= x + self.width() && my >= y && my <= y + self.height()
    }
    
}
