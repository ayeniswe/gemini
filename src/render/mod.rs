//! The `render` module provides abstractions for rendering UI elements.
//!
//! This module defines the [`Renderer`] trait, which acts as a blueprint for different rendering backends. It allows for rendering individual UI components, clearing the screen, and presenting the final rendered image to the display.
//!
//! The purpose of this module is to provide flexibility in rendering strategies, enabling the UI framework to support different backends, such as `pixels`, `wgpu`, or software-based rendering implementations.
//!
//! You can implement this trait for any rendering system, and the UI framework will use it to display components consistently across different platforms and backends.

use crate::ui::widget::Widget;

pub mod pixels_backend;

/// A trait for rendering UI components.
///
/// Implementors of this trait define how to render individual UI widgets, clear
/// the screen, and present the final image.
///
/// This abstraction allows the UI framework to support multiple
/// rendering backends, such as `pixels`, `wgpu`, or software.
/// ```
pub trait Renderer {
    fn clear(&mut self);
    fn draw(&mut self, widget: &Box<dyn Widget>);
    fn present(&mut self);
}

/// Follows the row major formula
/// for indices mapping to a frame buffer with
/// RGBA channel
fn row_major(x: u32, y: u32, width: u32) -> usize {
    ((y * width + x) * 4) as usize
}
