use crate::{
    impl_hoverable, impl_widget,
    ui::{Hoverable, Widget},
};

use super::{action::hover::Hover, color::Color, style::Style, text::Text};

#[derive(Default, Debug, Clone)]
pub struct Layout {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

/// A basic rectangular UI component used within a `Grid` layout.
///
/// A `Cell` holds color, position, size, and hover state. It can be individually
/// styled and interacted with. Typically used as a grid element inside a `Canvas`.
#[derive(Default, Debug, Clone)]
pub struct Cell {
    pub text: Text,
    pub hover: Hover,
    pub style: Style,
    pub layout: Layout,
}
impl Cell {
    pub(crate) fn new(width: u32, height: u32) -> Self {
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
impl_widget! {Cell}
impl_hoverable! {Cell}

/// Represents a 2D grid of `Cell` widgets with uniform spacing.
///
/// Used within a `Canvas` to organize cells in rows and columns.
/// Spacing controls the number of divisions across both axes.
#[derive(Default, Debug, Clone)]
pub struct Grid {
    pub(crate) spacing: u32,
    pub(crate) cells: Vec<Vec<Cell>>,
}
