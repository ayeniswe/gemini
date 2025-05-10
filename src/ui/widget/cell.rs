use std::{
    any::Any,
    cell::{Ref, RefCell, RefMut},
};


use crate::action::Action;

use super::{impl_widget, BaseWidget, Widget};

/// A struct representing a cell in a grid.
///
/// The `Cell` struct represents a single unit in a grid layout. It
/// has the functionality of a `BaseWidget`, which includes common properties and 
/// behaviors for all widgets, while the cell itself is used as part of a
/// larger grid system for arranging and interacting with UI elements
///
/// This struct is typically used in a `Grid` layout where each `Cell`
/// acts as an individual container
/// for content or other widgets within that grid.
#[derive(Default, Clone)]
pub struct Cell {
    pub base: RefCell<BaseWidget>,
    pub actions: RefCell<Vec<Action>>,
}
impl Cell {
    pub fn new() -> Self {
        Cell::default()
    }
}
impl_widget! {Cell}
