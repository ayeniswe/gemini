use crate::{
    impl_hoverable, impl_widget,
    ui::{Hoverable, Widget},
};

use super::{action::hover::Hover, color::Color, layout::{Cell, Grid, Layout}, style::Style, text::Text};

/// A top-level widget that can optionally contain a grid of interactive cells.
///
/// A `Canvas` is a rectangular area that supports coloring, hovering, and optional
/// subdivision into a `Grid` via the `set_gridlines` method.
///
/// Used as a visual base or drawing area in UI compositions.
#[derive(Default, Debug, Clone)]
pub struct Canvas {
    pub text: Text,
    pub hover: Hover,
    pub style: Style,
    pub layout: Layout,
}
impl Canvas {
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
    /// Subdivides the canvas into a grid of equally sized `Cell` elements.
    ///
    /// This method generates a square grid of `spacing × spacing` cells,
    /// positioning and sizing each cell based on the canvas dimensions.
    pub fn set_gridlines(&mut self, spacing: u32) -> &mut Self {
        // Set all cell blocks
        let mut cells = vec![vec![Cell::default(); spacing as usize]; spacing as usize];
        let h_lines_spacing = self.layout.h / spacing;
        let w_lines_spacing = self.layout.h / spacing;
        for y in 0..spacing {
            for x in 0..spacing {
                let c = Cell::new(w_lines_spacing * (x + 1), h_lines_spacing * (y + 1));
                let c = c.set_x(x * w_lines_spacing).set_y(y * h_lines_spacing);
                cells[x as usize][y as usize] = c;
            }
        }

        self.style.grid = Some(Grid { spacing, cells });

        self
    }
}
impl_widget! {Canvas}
impl_hoverable! {Canvas}
