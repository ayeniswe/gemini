use std::{
    any::Any,
    cell::{Ref, RefCell, RefMut},
    sync::Arc,
};

use crate::{
    action::Action,
    ui::{
        color::Color,
        layout::{Col, Grid, Point, Row},
        sync::Thread,
    },
};

use super::{impl_widget, BaseWidget, Widget};

/// A struct representing a canvas widget.
///
/// The `Canvas` struct serves as a container for drawing, rendering, or
/// displaying graphical content. It has the functionality of a
/// `BaseWidget`, which includes common properties and behaviors for all
/// widgets, while providing a specific interface for graphical rendering.
///
/// The `Canvas` can be used as a drawing surface, allowing you to add
/// elements like shapes, images, or other visual content.
#[derive(Default, Clone)]
pub struct Canvas {
    pub base: RefCell<BaseWidget>,
    pub actions: RefCell<Vec<Action>>,
    emitter: Option<Arc<dyn Thread>>,
    pub grid: RefCell<Option<Grid>>,
}
impl Canvas {
    pub fn new() -> Self {
        Canvas::default()
    }
    /// Subdivides the canvas into a grid of equally sized `Cell` elements.
    ///
    /// This method generates a perfect square grid of `[size][size]` cells,
    /// positioning and sizing each cell based on the canvas dimensions.
    ///
    /// If `size` was 4 the grid dimension would be:
    /// ```
    /// | | | | |
    /// |x| | | |
    /// | | | | |
    /// | | | | |
    /// ```
    /// # Panics
    ///
    /// This function will panic if `size` is 0
    pub fn set_grid(mut self, size: u32, thickness: f64, color: Color) -> Self {
        let base = self.base.borrow_mut();

        self.grid = RefCell::new(Some(Grid::new(
            Point {
                x: size as f64,
                y: size as f64,
            },
            thickness,
            color.into(),
        )));

        drop(base);

        self
    }
    /// Set the actions to be triggered on every cell in
    /// the canvas grid
    ///
    /// NoOp if `set_grid` was not called before
    ///
    /// Cheaper to set them manullay with `set_cell_action`
    /// # Panics
    ///
    /// This function will panic if `Canvas` never called `set_width` or `set_height`
    pub fn set_cells_actions(self, actions: Vec<Action>) -> Self {
        if let Some(grid) = &*self.grid.borrow_mut() {
            for y in 0..grid.size.y as usize {
                for x in 0..grid.size.x as usize {
                    let cell = &grid.cells[y][x];
                    cell.base_mut().id = format!("{},{}", x, y);
                    for action in actions.iter().cloned() {
                        cell.action_mut().push(action);
                    }
                }
            }
        }

        self
    }
    /// Set an action to be triggered on a specific cell in
    /// the canvas grid
    ///
    /// NoOp if `set_grid` was not called before
    ///
    /// `pos` follows the origin at (0,0) at the top-left corner of the grid.
    /// `pos` (0,1) of a 4x4 grid would reference the second cell down starting from the top-left
    /// corner
    ///
    /// ```
    /// | | | | |
    /// |x| | | |
    /// | | | | |
    /// | | | | |
    /// ```
    /// # Panics
    ///
    /// This function will panic:
    /// - If `Canvas` never called `set_width` or `set_height`
    /// - If `pos` does not exist
    pub fn set_cell_action(self, pos: (Row, Col), action: Action) -> Self {
        if let Some(grid) = &*self.grid.borrow_mut() {
            let cell = &grid.cells[pos.0][pos.1];
            cell.action_mut().push(action);
        }

        self
    }
    /// Subdivides the canvas into a grid of equally sized `Cell` elements.
    ///
    /// This method generates a specific range grid of `[size.1][size.0]` cells,
    /// positioning and sizing each cell based on the canvas dimensions.
    ///
    /// If `size` was `(2, 4)` the grid dimension would be:
    /// ```
    /// | | |
    /// |x| |
    /// | | |
    /// | | |
    /// ```
    /// # Panics
    ///
    /// This function will panic if `size.0` or `size.1` is 0
    pub fn set_grid_range(mut self, size: (u32, u32), thickness: f64, color: Color) -> Self {
        let base = self.base.borrow_mut();

        self.grid = RefCell::new(Some(Grid::new(
            Point {
                x: size.0 as f64,
                y: size.1 as f64,
            },
            thickness,
            color.into(),
        )));

        drop(base);

        self
    }
}
impl_widget! {Canvas}

#[cfg(test)]
mod tests {
    use crate::ui::{color::Color, layout::Layout, widget::Widget};

    use super::Canvas;

    #[test]
    fn test_gridlines_are_spaced_correctly() {
        let c = Canvas::new().set_width(32.0).set_height(16.0).set_grid(
            4,
            1.0,
            Color::RGBA(0, 0, 0, 0),
        );

        let mut grid = c.grid.borrow_mut().clone().unwrap();
        grid.resize(0.0, 0.0, 16.0, 32.0);

        let cells = grid.cells;
        assert!(
            cells[0][0].base.borrow().layout
                == Layout {
                    x: 0.0,
                    y: 0.0,
                    w: 8.0,
                    h: 4.0
                }
        );
        assert!(
            cells[1][0].base.borrow().layout
                == Layout {
                    x: 0.0,
                    y: 5.0,
                    w: 8.0,
                    h: 3.0
                }
        );
        assert!(
            cells[2][0].base.borrow().layout
                == Layout {
                    x: 0.0,
                    y: 9.0,
                    w: 8.0,
                    h: 3.0
                }
        );
        assert!(
            cells[3][0].base.borrow().layout
                == Layout {
                    x: 0.0,
                    y: 13.0,
                    w: 8.0,
                    h: 3.0
                }
        );
        assert!(
            cells[0][1].base.borrow().layout
                == Layout {
                    x: 9.0,
                    y: 0.0,
                    w: 7.0,
                    h: 4.0
                }
        );
        assert!(
            cells[1][1].base.borrow().layout
                == Layout {
                    x: 9.0,
                    y: 5.0,
                    w: 7.0,
                    h: 3.0
                }
        );
        assert!(
            cells[2][1].base.borrow().layout
                == Layout {
                    x: 9.0,
                    y: 9.0,
                    w: 7.0,
                    h: 3.0
                }
        );
        assert!(
            cells[3][1].base.borrow().layout
                == Layout {
                    x: 9.0,
                    y: 13.0,
                    w: 7.0,
                    h: 3.0
                }
        );
        assert!(
            cells[0][2].base.borrow().layout
                == Layout {
                    x: 17.0,
                    y: 0.0,
                    w: 7.0,
                    h: 4.0
                }
        );
        assert!(
            cells[1][2].base.borrow().layout
                == Layout {
                    x: 17.0,
                    y: 5.0,
                    w: 7.0,
                    h: 3.0
                }
        );
        assert!(
            cells[2][2].base.borrow().layout
                == Layout {
                    x: 17.0,
                    y: 9.0,
                    w: 7.0,
                    h: 3.0
                }
        );
        assert!(
            cells[3][2].base.borrow().layout
                == Layout {
                    x: 17.0,
                    y: 13.0,
                    w: 7.0,
                    h: 3.0
                }
        );
        assert!(
            cells[0][3].base.borrow().layout
                == Layout {
                    x: 25.0,
                    y: 0.0,
                    w: 7.0,
                    h: 4.0
                }
        );
        assert!(
            cells[1][3].base.borrow().layout
                == Layout {
                    x: 25.0,
                    y: 5.0,
                    w: 7.0,
                    h: 3.0
                }
        );
        assert!(
            cells[2][3].base.borrow().layout
                == Layout {
                    x: 25.0,
                    y: 9.0,
                    w: 7.0,
                    h: 3.0
                }
        );
        assert!(
            cells[3][3].base.borrow().layout
                == Layout {
                    x: 25.0,
                    y: 13.0,
                    w: 7.0,
                    h: 3.0
                }
        );
    }
}
