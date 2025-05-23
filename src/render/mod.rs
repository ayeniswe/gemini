//! The `render` module provides abstractions for rendering UI elements.
//!
//! This module defines the [`Renderer`] trait, which acts as a blueprint for different rendering backends. 
//! It allows for rendering individual UI components, clearing the screen, and presenting the final rendered image to the display.
//!
//! The purpose of this module is to provide flexibility in rendering strategies, enabling the 
//! UI framework to support different backends, such as `pixels`, `wgpu`, or software-based rendering implementations.
//!
//! You can implement this trait for any rendering system, and the UI framework will 
//! use it to display components consistently across different platforms and backends.

use std::rc::Rc;

use crate::ui::widget::WidgetI;

pub mod pixels_backend;
pub mod pre;

/// A trait for rendering UI components.
///
/// Implementors of this trait define how to render individual UI widgets, clear
/// the screen, and present the final image.
///
/// This abstraction allows the UI framework to support multiple
/// rendering backends, such as `pixels`, `wgpu`, or software.
/// ```
pub trait Renderer {
    /// Clears a rect region
    fn dirty_clear(&mut self, x: f64, y: f64, h: f64, w: f64);
    /// Clears the entire screen
    fn clear(&mut self);
    /// Draw all widgets to screen
    fn draw(&mut self, widget: &Rc<dyn WidgetI>);
    /// Show the drawings
    fn present(&mut self);
}

/// Follows the row major formula
/// for indices mapping to a frame buffer with
/// RGBA channel
fn row_major(x: u32, y: u32, width: u32) -> usize {
    y.saturating_mul(width).saturating_add(x).saturating_mul(4) as usize
}
