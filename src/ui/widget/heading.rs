use std::{
    any::Any,
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
    sync::Arc,
};

use crate::{
    action::Action,
    ui::sync::{Thread, Trigger},
};

use super::{impl_widget, BaseWidget, Widget, WidgetI, WidgetInternal};

/// A struct representing a heading widget.
///
/// The `Heading` struct represents a heading h1 h2 h3 UI element, typically used
/// for header above section.
///
/// The `Heading` struct includes extra functionalities specific to
/// headings
#[derive(Default, Clone)]
pub struct Heading {
    pub base: RefCell<BaseWidget>,
    pub actions: RefCell<Vec<Action>>,
    emitter: Option<Arc<dyn Thread>>,
    trigger: RefCell<Option<Rc<Trigger>>>,
}
impl Heading {
    pub fn new() -> Self {
        Heading::default()
    }
    pub fn set_large_heading(self) -> Self {
        self.base.borrow_mut().text.font_size = 32.0;
        self
    }
    pub fn set_medium_heading(self) -> Self {
        self.base.borrow_mut().text.font_size = 24.0;
        self
    }
    pub fn set_small_heading(self) -> Self {
        self.base.borrow_mut().text.font_size = 18.0;
        self
    }
}
impl_widget! {Heading}
