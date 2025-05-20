use std::{
    any::Any,
    cell::{Ref, RefCell, RefMut},
    sync::Arc,
};

use crate::{
    action::Action,
    ui::{color::LIGHT_GRAY, sync::Thread},
};

use super::{impl_widget, BaseWidget, Widget};

/// Scrollbar thickness
const SCROLLBAR_SIZE: f64 = 10.0;
const SCROLLBAR_BUFFER: f64 = 5.0;

/// The `Scrollbar` struct represents a scrollbar along the
/// x or y axis of a `Container`.
///
/// The `Scrollbar` can only be used with a `Container` widget.
#[derive(Default, Clone)]
pub struct ScrollBar {
    pub base: RefCell<BaseWidget>,
    pub actions: RefCell<Vec<Action>>,
    emitter: Option<Arc<dyn Thread>>,
    pub(crate) buffer: f64,
}
impl ScrollBar {
    /// Create a new `Scrollbar` on the y-axis
    pub fn new_y() -> Self {
        let mut scrollbar = ScrollBar::default();
        scrollbar.buffer = SCROLLBAR_BUFFER;

        {
            let mut scrollbar_base = scrollbar.base_mut();
            scrollbar_base.layout.w = SCROLLBAR_SIZE;
            scrollbar_base.layout.h = -1.0; // hides it to not be clickable
            scrollbar_base.style.color = LIGHT_GRAY.into();
        }

        scrollbar
    }
    /// Create a new `Scrollbar` on the x-axis
    pub fn new_x() -> Self {
        let mut scrollbar = ScrollBar::default();
        scrollbar.buffer = SCROLLBAR_BUFFER;

        {
            let mut scrollbar_base = scrollbar.base_mut();
            scrollbar_base.layout.h = SCROLLBAR_SIZE;
            scrollbar_base.layout.w = -1.0; // hides it to not be clickable
            scrollbar_base.style.color = LIGHT_GRAY.into()
        }

        scrollbar
    }
}
impl_widget! {ScrollBar}
