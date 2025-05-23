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
/// A struct representing a button widget.
///
/// The `Button` struct encapsulates a button UI element, typically used
/// for user interactions. It has the functionality of a `BaseWidget`,
/// which includes common properties and behaviors for all widgets, while
/// adding button-specific logic and styling.
///
/// The `Button` can be used in graphical user interfaces or any context
/// where a button-like interaction is needed.
#[derive(Default, Clone)]
pub struct Button {
    pub base: RefCell<BaseWidget>,
    pub actions: RefCell<Vec<Action>>,
    emitter: Option<Arc<dyn Thread>>,
    trigger: RefCell<Option<Rc<Trigger>>>,
}
impl Button {
    pub fn new() -> Self {
        Button::default()
    }
}
impl_widget! {Button}
