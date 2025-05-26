//! A module providing foundational UI widgets and layout components.
//!
//! This module defines the core building blocks for a user interface system,
//! including reusable and composable widgets. Each widget is built on top of
//! `BaseWidget`, which provides shared properties like layout, style, text,
//! and action handling.

//! ## Design Philosophy
//! Widgets in this module are lightweight, composable, and declarative.
//! By separating layout, styling, and behavior, the system promotes clean
//! abstractions and flexible UI composition.
//!
//! This module is intended to serve as a UI foundation for applications
//! requiring customizable and structured graphical interfaces.

use std::{
    any::Any,
    cell::{Ref, RefMut},
    rc::Rc,
    sync::Arc,
};

use crate::action::Action;

use super::{
    color::{Color, ColorState},
    layout::{Layout, Point},
    state::State,
    style::Style,
    sync::{Thread, Trigger},
    text::Text,
};

pub mod button;
pub mod canvas;
pub(crate) mod cell;
pub mod container;
pub mod heading;
pub mod label;
pub mod scrollbar;

/// A base struct representing a generic UI widget.
///
/// The `BaseWidget` struct serves as the foundation for UI elements,
/// providing core properties common to most widgets. These properties
/// include text content, styling, layout information, and actions
/// (events or behavior triggers). It is meant to be  extended by other
/// widgets to build more specific functionality and behavior.
///
/// - `text`: Represents the text content of the widget. This could be used
///   to display labels, descriptions, or other textual information on
///   the widget.
/// - `style`: Defines the visual styling of the widget, such as colors,
///   borders, fonts, padding, and other style-related attributes.
/// - `layout`: Contains the layout properties of the widget, such as its
///   size, position, and alignment within a container or layout grid.
/// - `actions`: A collection of actions associated with the widget. Actions
///   represent the behaviors or events that the widget can trigger or
///   respond to, such as clicks, hover events, or other interactions.
/// - `state`: A variety of transient visual states the widget is
///   currently in
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct BaseWidget {
    pub id: String,
    pub text: Text,
    pub style: Style,
    pub layout: Layout,
    pub offset: Point,
    pub state: State,
}

pub(crate) trait WidgetI: Widget + WidgetInternal {}

/// A trait representing special
/// internal methods known only to
/// internal widget use.
/// Basically the magic glue :).
pub(crate) trait WidgetInternal {
    /// Returns a internal cloned trigger for widget
    fn internal_trigger(&self) -> Option<Rc<Trigger>>;
    /// Returns a mutable internal trigger for widget
    fn internal_trigger_mut(&self) -> RefMut<'_, Option<Rc<Trigger>>>;
}
/// A trait representing a basic UI component.
///
/// Types that implement `Widget` can use fluent-style setters
/// for convenient method chaining.
///
/// Widget MUST be polymorphic since at runtime we have no clue
/// what widget could be used at the moment. Also to support one-offs
/// with downcasting, we must use the dirty `Any` trait bounds :(
pub trait Widget: Any + WidgetInternal {
    /// Allows downcasting to concrete types by returning a reference to `Any`.
    fn as_any(&self) -> &dyn Any;
    /// Returns an immutable reference to the list of actions associated with the widget.
    fn action(&self) -> Ref<'_, Vec<Action>>;
    /// Returns a mutable reference to the list of actions associated with the widget.
    fn action_mut(&self) -> RefMut<'_, Vec<Action>>;
    /// Returns an immutable reference to the widget's base properties.
    fn base(&self) -> Ref<'_, BaseWidget>;
    /// Returns a mutable reference to the widget's base properties.
    fn base_mut(&self) -> RefMut<'_, BaseWidget>;
    /// Returns an optional tied thread to run around this
    /// widget. The thread is a user custom thread manager
    /// that may exist
    fn emitter(&self) -> Option<&Arc<dyn Thread>>;
    /// Returns a trigger to aid for this widget redraws
    fn trigger(&self) -> Rc<Trigger> {
        self.internal_trigger()
            .expect("widget should be added to DOM")
    }
    /// Set the inside text for the widget
    fn set_label(self, label: &str) -> Self
    where
        Self: Sized,
    {
        self.base_mut().text.label = label.into();
        self
    }
    /// Set the font size of the inside text
    fn set_label_size(self, size: f32) -> Self
    where
        Self: Sized,
    {
        self.base_mut().text.font_size = size;
        self
    }
    /// Set a unique id for widget
    fn set_id(self, id: &str) -> Self
    where
        Self: Sized,
    {
        self.base_mut().id = id.into();
        self
    }
    /// Set the x-axis position of the widget
    fn set_x(self, x: f64) -> Self
    where
        Self: Sized,
    {
        self.base_mut().layout.x = x;
        self
    }
    /// Set the y-axis position of the widget
    fn set_y(self, y: f64) -> Self
    where
        Self: Sized,
    {
        self.base_mut().layout.y = y;
        self
    }
    /// Set the height dimension of the widget
    fn set_height(self, height: f64) -> Self
    where
        Self: Sized,
    {
        self.base_mut().layout.h = height;
        self
    }
    /// Set the width dimension of the widget
    fn set_width(self, width: f64) -> Self
    where
        Self: Sized,
    {
        self.base_mut().layout.w = width;
        self
    }
    /// Set the corner radius of the widget
    fn set_radius(self, radius: u32) -> Self
    where
        Self: Sized,
    {
        self.base_mut().style.radius = radius;
        self
    }
    /// Set the background color of the widget
    fn set_color(self, color: Color) -> Self
    where
        Self: Sized,
    {
        self.base_mut().style.color = ColorState::new(color);
        self
    }
    /// Align text in center vertically
    fn set_label_vertical(self) -> Self
    where
        Self: Sized,
    {
        self.base_mut().text.valign = true;
        self
    }
    /// Align text in center horizontally
    fn set_label_horizontal(self) -> Self
    where
        Self: Sized,
    {
        self.base_mut().text.halign = true;
        self
    }
    /// Sets a trigger action for the widget
    ///
    /// See `Action` enum for the types of actions avaliable
    fn on_action(self, action: Action) -> Self
    where
        Self: Sized,
    {
        self.action_mut().push(action);
        self
    }
    /// Allows an emitter to be attached to this widget instance
    /// for triggering updates to the widget when signals come
    /// through UI event proxy
    fn connect<T: Thread + 'static>(self, emitter: T) -> Self
    where
        Self: Sized;
}

/// Implements the [`Widget`] trait for a struct with common UI fields.
///
/// This macro generates an implementation of the `Widget` trait for
/// a given type, assuming it has the `base` and `actions` field.
macro_rules! impl_widget {
    ($type:ty) => {
        impl Widget for $type {
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn base(&self) -> Ref<'_, BaseWidget> {
                self.base.borrow()
            }
            fn base_mut(&self) -> RefMut<'_, BaseWidget> {
                self.base.borrow_mut()
            }
            fn action(&self) -> Ref<'_, Vec<Action>> {
                self.actions.borrow()
            }
            fn action_mut(&self) -> RefMut<'_, Vec<Action>> {
                self.actions.borrow_mut()
            }
            fn emitter(&self) -> Option<&Arc<dyn Thread>> {
                self.emitter.as_ref()
            }
            fn connect<T: Thread + 'static>(mut self, emitter: T) -> Self {
                self.emitter = Some(Arc::new(emitter));
                self
            }
        }
        impl WidgetInternal for $type {
            fn internal_trigger(&self) -> Option<Rc<Trigger>> {
                self.trigger.borrow().clone()
            }
            fn internal_trigger_mut(&self) -> RefMut<'_, Option<Rc<Trigger>>> {
                self.trigger.borrow_mut()
            }
        }
        impl WidgetI for $type {}
    };
}
pub(crate) use impl_widget;
