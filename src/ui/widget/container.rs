use std::{
    any::Any,
    cell::{Ref, RefCell, RefMut},
};

use crate::action::Action;

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
    pub children: Vec<Box<dyn Widget>>,
    valign: bool,
    halign: bool,
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
    pub fn add_widget<T: Widget + 'static>(mut self, widget: T) -> Self {
        if self.halign {
            let new_x = {
                let child = widget.base();
                let h_middle = (self.base.borrow().layout.w / 2).abs_diff(child.layout.w / 2);
                child.layout.x + h_middle
            };

            widget.base_mut().layout.x = new_x;
        }
        if self.halign {
            let new_y = {
                let child = widget.base();
                let v_middle = (self.base.borrow().layout.h / 2).abs_diff(child.layout.h / 2);
                child.layout.y + v_middle
            };

            widget.base_mut().layout.y = new_y;
        }

        self.children.push(Box::new(widget));
        self
    }
}
impl_widget! {Container}
