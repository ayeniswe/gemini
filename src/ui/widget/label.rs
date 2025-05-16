use std::{
    any::Any,
    cell::{Ref, RefCell, RefMut},
    sync::Arc,
};

use crate::{action::Action, ui::sync::Thread};

use super::{impl_widget, BaseWidget, Widget};

/// A struct representing a heading widget.
///
/// The `Label` struct represents basic text creation
/// such as paragraph, etc
#[derive(Default, Clone)]
pub struct Label {
    pub base: RefCell<BaseWidget>,
    pub actions: RefCell<Vec<Action>>,
    emitter: Option<Arc<dyn Thread>>,
}
impl Label {
    pub fn new() -> Self {
        Label::default()
    }
}
impl_widget! {Label}
