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
/// The `Label` struct represents basic text creation
/// such as paragraph, etc
#[derive(Default, Clone)]
pub struct Label {
    pub base: RefCell<BaseWidget>,
    pub actions: RefCell<Vec<Action>>,
    emitter: Option<Arc<dyn Thread>>,
    trigger: RefCell<Option<Rc<Trigger>>>,
}
impl Label {
    pub fn new() -> Self {
        Label::default()
    }
}
impl_widget! {Label}
