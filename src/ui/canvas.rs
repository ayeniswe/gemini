use std::{cell::RefCell, rc::Rc};

use crate::{
    impl_hoverable, impl_widget,
    ui::{Hoverable, Widget},
};

use super::color::Color;

/// A basic rectangular UI component used within a `Grid` layout.
///
/// A `Cell` holds color, position, size, and hover state. It can be individually
/// styled and interacted with. Typically used as a grid element inside a `Canvas`.
#[derive(Default, Debug, Clone)]
pub struct Cell {
    pub label: Option<String>,
    pub hover_color: Option<Color>,
    pub color: Color,
    pub hovered: bool,
    pub pos: (u32, u32),
    pub width: u32,
    pub height: u32,
    pub radius: u32,
}
impl Cell {
    fn new(width: u32, height: u32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            height,
            width,
            ..Default::default()
        }))
    }
}
impl_widget! {Cell}
impl_hoverable! {Cell}

/// Represents a 2D grid of `Cell` widgets with uniform spacing.
///
/// Used within a `Canvas` to organize cells in rows and columns.
/// Spacing controls the number of divisions across both axes.
#[derive(Default, Debug, Clone)]
pub struct Grid {
    pub(crate) spacing: u32,
    cells: Vec<Vec<Rc<RefCell<Cell>>>>,
}

/// A top-level widget that can optionally contain a grid of interactive cells.
///
/// A `Canvas` is a rectangular area that supports coloring, hovering, and optional
/// subdivision into a `Grid` via the `set_gridlines` method.
///
/// Used as a visual base or drawing area in UI compositions.
#[derive(Default, Debug, Clone)]
pub struct Canvas {
    pub label: Option<String>,
    pub hover_color: Option<Color>,
    pub color: Color,
    pub hovered: bool,
    pub pos: (u32, u32),
    pub width: u32,
    pub height: u32,
    pub radius: u32,
    pub grid: Option<Grid>,
}
impl Canvas {
    pub fn new(width: u32, height: u32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            height,
            width,
            ..Default::default()
        }))
    }
    /// Subdivides the canvas into a grid of equally sized `Cell` elements.
    ///
    /// This method generates a square grid of `spacing × spacing` cells,
    /// positioning and sizing each cell based on the canvas dimensions.
    pub fn set_gridlines(&mut self, spacing: u32) -> &mut Self {
        // Set all cell blocks
        let mut cells =
            vec![vec![Rc::new(RefCell::new(Cell::default())); spacing as usize]; spacing as usize];
        let h_lines_spacing = self.height / spacing;
        let w_lines_spacing = self.width / spacing;
        for y in 0..spacing {
            for x in 0..spacing {
                let c = Cell::new(w_lines_spacing * (x + 1), h_lines_spacing * (y + 1));
                c.borrow_mut()
                    .set_x(x * w_lines_spacing)
                    .set_y(y * h_lines_spacing);
                cells[x as usize][y as usize] = c;
            }
        }

        self.grid = Some(Grid { spacing, cells });

        self
    }
}
impl_widget! {Canvas}
impl_hoverable! {Canvas}
