use std::default;

use crate::ui::widget::cell::Cell;

/// A struct representing the position and size of a UI element.
///
/// The `Layout` struct encapsulates the layout properties for a UI element,
/// including its position (`x`, `y`) and size (`w`, `h`). It is used
/// as a composition field within other widgets to define their placement
/// and dimensions within a UI layout or container.
///
/// - `x`: The horizontal position (offset) of the widget relative to its
/// container or parent.
/// - `y`: The vertical position (offset) of the widget relative to its
/// container or parent.
/// - `w`: The width of the widget, defining how wide it is within its
/// container.
/// - `h`: The height of the widget, defining how tall it is within its
/// container.
///
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Layout {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}
impl Layout {
    /// Determines if mouse is  in the bounds of this
    /// layout
    pub(crate) fn is_inbounds(&self, mx: f64, my: f64) -> bool {
        mx >= self.x as f64
            && mx <= (self.x + self.w) as f64
            && my >= self.y as f64
            && my <= (self.y + self.h) as f64
    }
    /// Determines the center of the layout vertically
    /// with the `rhs` included in the layout
    pub(crate) fn vertical_center(&self, rhs: u32) -> u32 {
        (self.h.saturating_sub(rhs)) / 2
    }
    /// Determines the center of the layout horizontally
    /// with the `rhs` included in the layout
    pub(crate) fn horizontal_center(&self, rhs: u32) -> u32 {
        (self.w.saturating_sub(rhs)) / 2
    }
}

/// The `Point` struct defines a simple x and y coordinates
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}
impl From<ab_glyph::Point> for Point {
    fn from(value: ab_glyph::Point) -> Self {
        Point {
            x: value.x as u32,
            y: value.y as u32,
        }
    }
}

/// A struct representing a grid layout for UI elements.
///
/// The `Grid` struct is designed to manage a 2D grid of `Cell` elements,
/// where each `Cell` represents a unit within the grid. This struct provides a way to
/// organize widgets or other UI elements in a structured, grid-based layout
/// with customizable spacing between cells.
///
/// - `size`: Defines the spacing between adjacent cells in the grid.
///   This value controls the gap between rows and columns of cells.
/// - `cells`: A 2D vector (`Vec<Vec<Cell>>`) representing the grid's cells.
///   Each `Cell` contains a UI widget or content that is arranged in the
///   grid's structure. The dimensions of this vector define the grid's rows
///   and columns.
#[derive(Default, Clone)]
pub struct Grid {
    pub(crate) size: Point,
    pub(crate) cells: Vec<Vec<Cell>>,
    pub(crate) thickness: u32,
}
impl Grid {
    /// Create a new `Grid` filling the `cells`
    /// with an empty widget with size `[size.y][size.x]`
    pub(crate) fn new(size: Point, thickness: u32) -> Self {
        Self {
            size,
            cells: vec![vec![Cell::default(); size.x as usize]; size.y as usize],
            thickness,
        }
    }
    /// Resize grid to meet the dimensions of
    /// `height x width` also account for pos `x` and `y` offset
    pub(crate) fn resize(&mut self, x: u32, y: u32, height: u32, width: u32) {
        let h_cell_size = height / self.size.y;
        let w_cell_size = width / self.size.x;

        self.on_cell(|pos, c| {
            let mut cbase = c.base.borrow_mut();
            // Due to line thickness being at minimal 1 px we need to
            // account for that spacing that way we do not overlap
            // cells
            let buffer_x = pos.x * w_cell_size;
            let buffer_y = pos.y * h_cell_size;
            cbase.layout.x = if buffer_x > 0 {
                buffer_x + self.thickness
            } else {
                0
            } + x;
            cbase.layout.y = if buffer_y > 0 {
                buffer_y + self.thickness
            } else {
                0
            } + y;
            cbase.layout.w = if buffer_x > 0 {
                w_cell_size - self.thickness
            } else {
                w_cell_size
            };
            cbase.layout.h = if buffer_y > 0 {
                h_cell_size - self.thickness
            } else {
                h_cell_size
            };
        });
    }
    /// Callback function on every cell
    ///
    /// Callback receives the 2D indices pos `Point` as well as
    /// the concrete Cell instance
    pub(crate) fn on_cell<F: FnMut(Point, &Cell)>(&self, mut callback: F) {
        for y in 0..self.size.y as usize {
            for x in 0..self.size.x as usize {
                let cell = &self.cells[y][x];
                callback(
                    Point {
                        x: x as u32,
                        y: y as u32,
                    },
                    cell,
                )
            }
        }
    }
}

pub type Row = usize;
pub type Col = usize;

/// The `FlexLayout` provides a variety of ways to organize
/// the container of widgets in a uniform way
#[derive(Default, Clone)]
pub enum FlexLayout {
    #[default]
    /// Default for `Container` widget
    None,
    /// Layout a container as a grid with a specific amount of
    /// columns
    ///
    /// ## Example
    /// ```
    /// let mut central_panel = Container::new().set_flex_layout(FlexLayout::FlexGrid(4))
    /// ```
    /// 
    /// How the layout would look if 5 widgets 
    /// were stored in the container:
    /// 
    /// ```
    /// -----------------
    /// | w | w | w | w |
    /// | w |            
    /// -----------------                   
    /// ```
    /// 
    /// # Panics
    /// 
    /// If the `Col` specified is 0 it will panic
    FlexGrid(Col),
}
