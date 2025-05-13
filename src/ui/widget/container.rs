use std::{
    any::Any,
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
    sync::Arc,
};

use crate::{
    action::Action,
    ui::{
        layout::{Col, FlexLayout},
        sync::Thread,
    },
};

use super::{impl_widget, BaseWidget, Widget};

/// A struct representing a container widget.
///
/// The `Container` struct allows other widgets
/// to be contained within itself. This widget
/// should primarily be used when more than a single
/// widgets need to sit nicely in a certain space
/// for user interactions. It has the functionality of a `BaseWidget`,
/// which includes common properties and behaviors for all widgets.
#[derive(Default)]
pub struct Container {
    pub base: RefCell<BaseWidget>,
    pub actions: RefCell<Vec<Action>>,
    emitter: Option<Arc<dyn Thread>>,
    pub children: Vec<Rc<dyn Widget>>,
    pub flex: FlexLayout,
    valign: bool,
    halign: bool,
    gap: u32,
}
impl Container {
    pub fn new() -> Self {
        Container::default()
    }
    /// Align children in the container along
    /// the y-axis
    pub fn set_vertical(mut self) -> Self {
        self.valign = true;
        self
    }
    /// Align children in the container along
    /// the x-axis
    pub fn set_horizontal(mut self) -> Self {
        self.halign = true;
        self
    }
    /// Set a gap size between every child in container
    pub fn set_gap(mut self, gap: u32) -> Self {
        self.gap = gap;
        self
    }
    /// Set the type of flex layout to use
    pub fn set_flex_layout(mut self, layout: FlexLayout) -> Self {
        self.flex = layout;
        self
    }
    /// Sets up a flex grid layout to allows widgets
    /// to organize widgets in grid flow fashion
    pub(crate) fn create_flex_grid_layout(&self, cols: Col) {
        assert!(cols > 0);

        let mut prev: Option<&Rc<dyn Widget>> = None;

        let mut row = 0;
        let mut col = 0;

        let rows = self.children.len().div_ceil(cols) as u32;

        let gaps_factor_col = self.gap * (rows - 1);
        let gaps_factor_row = self.gap * (cols as u32 - 1);

        let width_share = (self.base.borrow().layout.w / cols as u32) - gaps_factor_row;
        let height_share = (self.base.borrow().layout.h / rows) - gaps_factor_col;

        for child in self.children.iter().enumerate() {
            let (idx, child) = child;

            // Space out grid layout
            // to meet max columns and row
            if idx != 0 {
                if idx as u32 % cols as u32 == 0 {
                    row += 1;
                    col = 0;
                } else {
                    col += 1;
                }
            }

            ////////////
            /////// OVERFLOWING PROTECTION
            ////
            let new_w = { child.base().layout.w.min(width_share) };
            child.base_mut().layout.w = new_w;
            let new_h = { child.base().layout.h.min(height_share) };
            child.base_mut().layout.h = new_h;

            ////////////
            /////// ALIGMENT
            ////
            if self.halign {
                let new_x = {
                    let child_base = child.base();

                    // The possibility of other columns spaces being filled
                    let cols_max_spacing = child_base.layout.w * cols as u32;
                    // The full total spacing a grid row could take
                    let max_row_spacing = cols_max_spacing + gaps_factor_row;

                    child_base.layout.x + self.base().layout.horizontal_center(max_row_spacing)
                };
                child.base_mut().layout.x = new_x;
            }
            if self.valign {
                let new_y = {
                    let child_base = child.base();

                    // The possibility of other rows spaces being filled
                    let rows_max_spacing = child_base.layout.h * rows;
                    // The full total spacing a grid column could take
                    let max_col_spacing = rows_max_spacing + gaps_factor_col;

                    child_base.layout.y + self.base().layout.vertical_center(max_col_spacing)
                };
                child.base_mut().layout.y = new_y;
            }

            ////////////
            /////// GRID LAYOUT
            ////
            if let Some(prev) = prev {
                let mut child_base = child.base_mut();
                child_base.layout.x =
                    (col * (prev.base().layout.w + self.gap)) + child_base.layout.x;
                child_base.layout.y = row * (prev.base().layout.h + self.gap) + child_base.layout.y;
            }

            prev = Some(child)
        }
    }
    pub fn add_widget<T: Widget + 'static>(&mut self, widget: T) {
        self.children.push(Rc::new(widget));
    }
}
impl_widget! {Container}
