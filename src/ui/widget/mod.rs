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

use crate::action::Action;

use super::{
    color::Color,
    layout::{Grid, Layout},
    style::Style,
    text::Text,
};

pub mod button;
pub mod canvas;
pub(crate) mod cell;

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
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct BaseWidget {
    pub text: Text,
    pub style: Style,
    pub layout: Layout,
    pub actions: Vec<Action>,
}

/// A trait representing a basic UI component.
///
/// Types that implement `Widget` can use fluent-style setters
/// for convenient method chaining.
///
pub trait Widget {
    fn grid(&self) -> &Option<Grid>;
    fn x(&self) -> u32;
    fn x_mut(&mut self) -> &mut u32;
    fn y(&self) -> u32;
    fn y_mut(&mut self) -> &mut u32;
    fn height(&self) -> u32;
    fn height_mut(&mut self) -> &mut u32;
    fn width(&self) -> u32;
    fn width_mut(&mut self) -> &mut u32;
    fn radius(&self) -> u32;
    fn radius_mut(&mut self) -> &mut u32;
    fn label(&self) -> &str;
    fn label_mut(&mut self) -> &mut String;
    fn color(&self) -> &Color;
    fn color_mut(&mut self) -> &mut Color;
    fn action_mut(&mut self) -> &mut Vec<Action>;
    fn set_label(mut self, label: &str) -> Self
    where
        Self: Sized,
    {
        *self.label_mut() = label.into();
        self
    }
    fn set_x(mut self, x: u32) -> Self
    where
        Self: Sized,
    {
        *self.x_mut() = x;
        self
    }
    fn set_y(mut self, y: u32) -> Self
    where
        Self: Sized,
    {
        *self.y_mut() = y;
        self
    }
    fn set_height(mut self, height: u32) -> Self
    where
        Self: Sized,
    {
        *self.height_mut() = height;
        self
    }
    fn set_width(mut self, width: u32) -> Self
    where
        Self: Sized,
    {
        *self.width_mut() = width;
        self
    }
    fn set_radius(mut self, radius: u32) -> Self
    where
        Self: Sized,
    {
        *self.radius_mut() = radius;
        self
    }
    fn set_color(mut self, color: Color) -> Self
    where
        Self: Sized,
    {
        *self.color_mut() = color;
        self
    }
    fn on_action(mut self, action: Action) -> Self
    where
        Self: Sized,
    {
        self.action_mut().push(action);
        self
    }
}

/// Implements the [`Widget`] trait for a struct with common UI fields.
///
/// This macro generates an implementation of the `Widget` trait for
/// a given type, assuming it has the `base` field.
macro_rules! impl_widget {
    ($type:ty) => {
        impl Widget for $type {
            fn grid(&self) -> &Option<Grid> {
                &self.base.style.grid
            }
            fn x(&self) -> u32 {
                self.base.layout.x
            }
            fn x_mut(&mut self) -> &mut u32 {
                &mut self.base.layout.x
            }
            fn y(&self) -> u32 {
                self.base.layout.y
            }
            fn y_mut(&mut self) -> &mut u32 {
                &mut self.base.layout.y
            }
            fn height(&self) -> u32 {
                self.base.layout.h
            }
            fn height_mut(&mut self) -> &mut u32 {
                &mut self.base.layout.h
            }
            fn width(&self) -> u32 {
                self.base.layout.w
            }
            fn width_mut(&mut self) -> &mut u32 {
                &mut self.base.layout.w
            }
            fn label(&self) -> &str {
                &self.base.text.label
            }
            fn label_mut(&mut self) -> &mut String {
                &mut self.base.text.label
            }
            fn color(&self) -> &Color {
                &self.base.style.color
            }
            fn color_mut(&mut self) -> &mut Color {
                &mut self.base.style.color
            }
            fn radius(&self) -> u32 {
                self.base.style.radius
            }
            fn radius_mut(&mut self) -> &mut u32 {
                &mut self.base.style.radius
            }
            fn action_mut(&mut self) -> &mut Vec<Action> {
                &mut self.base.actions
            }
        }
    };
}
pub(crate) use impl_widget;
