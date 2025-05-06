use crate::{
    action::Action,
    ui::{layout::Grid, Color},
};

use super::{impl_widget, BaseWidget, Widget};

/// A struct representing a cell in a grid.
///
/// The `Cell` struct represents a single unit in a grid layout. It
/// has the functionality of a `BaseWidget`, which includes common properties and behaviors
/// for all widgets, while the cell itself is used as part of a
/// larger grid system for arranging and interacting with UI elements
///
/// This struct is typically used in a `Grid` layout where each `Cell`
/// acts as an individual container
/// for content or other widgets within that grid.
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Cell {
    pub base: BaseWidget,
}
impl Cell {
    pub fn new() -> Self {
        Cell::default()
    }
}
impl_widget! {Cell}
