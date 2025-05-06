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

/// A struct representing a grid layout for UI elements.
///
/// The `Grid` struct is designed to manage a 2D grid of `Cell` elements, 
/// where each `Cell` represents a unit within the grid. This struct provides a way to
/// organize widgets or other UI elements in a structured, grid-based layout
/// with customizable spacing between cells.
///
/// - `spacing`: Defines the spacing between adjacent cells in the grid.
///   This value controls the gap between rows and columns of cells.
/// - `cells`: A 2D vector (`Vec<Vec<Cell>>`) representing the grid's cells.
///   Each `Cell` contains a UI widget or content that is arranged in the
///   grid's structure. The dimensions of this vector define the grid's rows
///   and columns.
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Grid {
    pub spacing: u32,
    pub cells: Vec<Vec<Cell>>,
}
