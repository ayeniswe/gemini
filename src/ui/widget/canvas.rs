use std::cell::{Ref, RefCell, RefMut};

use crate::{action::Action, ui::layout::Grid};

use super::{cell::Cell, impl_widget, BaseWidget, Widget};

/// A struct representing a canvas widget.
///
/// The `Canvas` struct serves as a container for drawing, rendering, or
/// displaying graphical content. It has the functionality of a
/// `BaseWidget`, which includes common properties and behaviors for all
/// widgets, while providing a specific interface for graphical rendering.
///
/// The `Canvas` can be used as a drawing surface, allowing you to add
/// elements like shapes, images, or other visual content.
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Canvas {
    pub base: RefCell<BaseWidget>,
    pub actions: RefCell<Vec<Action>>,
}
impl Canvas {
    pub fn new() -> Self {
        Canvas::default()
    }
    /// Subdivides the canvas into a grid of equally sized `Cell` elements.
    ///
    /// This method generates a square grid of `spacing × spacing` cells,
    /// positioning and sizing each cell based on the canvas dimensions.
    pub fn set_gridlines(&self, spacing: u32) -> &Self {
        let mut base = self.base.borrow_mut();

        // Set all cell blocks
        let mut cells = vec![vec![Cell::default(); spacing as usize]; spacing as usize];
        let h_lines_spacing = base.layout.h / spacing;
        let w_lines_spacing = base.layout.h / spacing;
        for y in 0..spacing {
            for x in 0..spacing {
                let c = Cell::new();
                {
                    let mut cbase = c.base.borrow_mut();
                    cbase.layout.w = (x + 1) * w_lines_spacing;
                    cbase.layout.h = h_lines_spacing * (y + 1);
                    cbase.layout.x = x * w_lines_spacing;
                    cbase.layout.y = h_lines_spacing * y;
                }
                cells[x as usize][y as usize] = c;
            }
        }

        base.style.grid = Some(Grid { spacing, cells });

        drop(base);

        self
    }
}
impl_widget! {Canvas}

#[cfg(test)]
mod tests {
    use crate::ui::layout::Layout;

    use super::Canvas;

    #[test]
    fn test_gridlines_are_spaced_correctly() {
        let c = Canvas::new();

        let mut cbase = c.base.borrow_mut();
        cbase.layout.w = 32;
        cbase.layout.h = 32;

        c.set_gridlines(4);

        let grid = cbase.style.grid.as_ref().unwrap();
        let cells = &grid.cells;

        assert!(
            cells[0][0].base.borrow().layout
                == Layout {
                    x: 0,
                    y: 0,
                    w: 8,
                    h: 8
                }
        );
        assert!(
            cells[0][1].base.borrow().layout
                == Layout {
                    x: 0,
                    y: 8,
                    w: 8,
                    h: 16
                }
        );
        assert!(
            cells[0][2].base.borrow().layout
                == Layout {
                    x: 0,
                    y: 16,
                    w: 8,
                    h: 24
                }
        );
        assert!(
            cells[0][3].base.borrow().layout
                == Layout {
                    x: 0,
                    y: 24,
                    w: 8,
                    h: 32
                }
        );
        assert!(
            cells[1][0].base.borrow().layout
                == Layout {
                    x: 8,
                    y: 0,
                    w: 16,
                    h: 8
                }
        );
        assert!(
            cells[1][1].base.borrow().layout
                == Layout {
                    x: 8,
                    y: 8,
                    w: 16,
                    h: 16
                }
        );
        assert!(
            cells[1][2].base.borrow().layout
                == Layout {
                    x: 8,
                    y: 16,
                    w: 16,
                    h: 24
                }
        );
        assert!(
            cells[1][3].base.borrow().layout
                == Layout {
                    x: 8,
                    y: 24,
                    w: 16,
                    h: 32
                }
        );
        assert!(
            cells[2][0].base.borrow().layout
                == Layout {
                    x: 16,
                    y: 0,
                    w: 24,
                    h: 8
                }
        );
        assert!(
            cells[2][1].base.borrow().layout
                == Layout {
                    x: 16,
                    y: 8,
                    w: 24,
                    h: 16
                }
        );
        assert!(
            cells[2][2].base.borrow().layout
                == Layout {
                    x: 16,
                    y: 16,
                    w: 24,
                    h: 24
                }
        );
        assert!(
            cells[2][3].base.borrow().layout
                == Layout {
                    x: 16,
                    y: 24,
                    w: 24,
                    h: 32
                }
        );
        assert!(
            cells[3][0].base.borrow().layout
                == Layout {
                    x: 24,
                    y: 0,
                    w: 32,
                    h: 8
                }
        );
        assert!(
            cells[3][1].base.borrow().layout
                == Layout {
                    x: 24,
                    y: 8,
                    w: 32,
                    h: 16
                }
        );
        assert!(
            cells[3][2].base.borrow().layout
                == Layout {
                    x: 24,
                    y: 16,
                    w: 32,
                    h: 24
                }
        );
        assert!(
            cells[3][3].base.borrow().layout
                == Layout {
                    x: 24,
                    y: 24,
                    w: 32,
                    h: 32
                }
        );
    }
}
