use std::rc::Rc;

use log::debug;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, Event, MouseButton, WindowEvent},
    window::Window,
};

use crate::ui::{
    sync::{Signal, Trigger},
    widget::{container::Container, Widget},
};

#[derive(Clone, Copy)]
enum Axis {
    /// X-axis scrollbar
    X,
    /// Y-axix scrollbar
    Y,
}
/// The `Scroll` struct allows `Container`s to have the ability
/// to respond to scroll movements on x or y axis
#[derive(Clone, Default, Copy)]
pub(crate) struct Scroll {
    /// The current selected scrollbar
    axis: Option<Axis>,
    /// The `cursor_offset` is so the scrollbar moves in uniform with
    /// the cursor no matter the area the cursor is place on the scrollbar.
    /// Prevents a snapping effect
    cursor_offset: f64,
    scroll_delta: f64,
    max_scroll_range: f64,
}
impl Scroll {
    /// Create a new `Scroll` action
    pub fn new() -> Self {
        Scroll::default()
    }
}
impl Scroll {
    fn on_pressed(&mut self, widget: &Container, last_cursor_pos: PhysicalPosition<f64>) {
        let widget_base = widget.base();
        let (x, y) = widget.scrollbar.as_ref().unwrap();

        // Determine the scrollbar selected
        let x_base = x.base();
        if x_base.state.hovered {
            debug!("x-axis scrollbar selected for widget: {}", widget_base.id);
            self.axis = Some(Axis::X);
            self.cursor_offset = last_cursor_pos.x - x_base.layout.x;
        }
        let y_base = y.base();
        if y_base.state.hovered {
            debug!("y-axis scrollbar selected for widget: {}", widget_base.id);
            self.axis = Some(Axis::Y);
            self.cursor_offset = last_cursor_pos.y - y_base.layout.y;
        }
    }
    /// We must determine the accurate range of scroll so content
    /// can be adjusted in uniform
    ///
    /// The delta is needed in order to apply the right shift increments
    /// on the child content
    fn compute_scroll(&mut self, widget: &Container) {
        let widget_base = widget.base();
        let (x, y) = widget.scrollbar.as_ref().unwrap();

        match self.axis {
            Some(Axis::X) => {
                let x_base = x.base();

                let container_width = widget_base.layout.w + widget_base.layout.x;
                let overflow_x = widget
                    .children
                    .iter()
                    .fold(container_width, |acc, child| child.base().layout.w.max(acc));
                let total_overflow = overflow_x - container_width;
                let scrollbar_buffer = x_base.layout.w;
                let total_scroll_range = container_width;
                let true_scroll_range = total_scroll_range - scrollbar_buffer;
                let content_offset = widget_base.layout.x;
                let delta = total_overflow / (true_scroll_range - content_offset - y.buffer);

                self.max_scroll_range = true_scroll_range;
                self.scroll_delta = delta;

                debug!(
                    "x scroll range - '{}' detected for widget: {}",
                    true_scroll_range, widget_base.id
                );
            }
            Some(Axis::Y) => {
                let y_base = y.base();

                let last_child = &widget.children[widget.children.len() - 1];
                let last_child_base = last_child.base();

                let overflow_y = last_child_base.layout.y + last_child_base.layout.h;
                let container_height = widget_base.layout.h + widget_base.layout.y;
                let total_overflow = overflow_y - container_height;
                let scrollbar_buffer = y_base.layout.h;
                let total_scroll_range = container_height;
                let true_scroll_range = total_scroll_range - scrollbar_buffer;
                let content_offset = widget_base.layout.y;
                let delta = total_overflow / (true_scroll_range - content_offset - x.buffer);

                self.max_scroll_range = true_scroll_range;
                self.scroll_delta = delta;

                debug!(
                    "y scroll range - '{}' detected for widget: {}",
                    true_scroll_range, widget_base.id
                );
            }
            _ => unreachable!(),
        }
    }
    fn on_cursor_movement(&self, widget: &Container, pos: PhysicalPosition<f64>) {
        let (x, y) = widget.scrollbar.as_ref().unwrap();

        // Determine if in view
        let ishovered = x.base().layout.is_inbounds(pos.x, pos.y);
        if ishovered {
            x.base_mut().state.hovered = true;
            debug!(
                "triggered hover for x scrollbar for widget: {}",
                widget.base().id
            );
        }
        let ishovered = y.base().layout.is_inbounds(pos.x, pos.y);
        if ishovered {
            y.base_mut().state.hovered = true;
            debug!(
                "triggered hover for y scrollbar for widget: {}",
                widget.base().id
            );
        }
    }
    fn on_scroll_movement(&self, widget: &Container, pos: PhysicalPosition<f64>) {
        let (x, y) = widget.scrollbar.as_ref().unwrap();

        match self.axis {
            Some(Axis::X) => {
                let mut x_base = x.base_mut();
                let widget_base = widget.base();

                // Move scrollbar
                x_base.layout.x =
                    (pos.x - self.cursor_offset).clamp(widget_base.layout.x, self.max_scroll_range);

                // Move container content
                let shift = (x_base.layout.x - widget_base.layout.x) * self.scroll_delta;
                for child in &widget.children {
                    child.base_mut().offset.x = -shift;
                }

                debug!(
                    "applying -{}px xshift offset to content for widget: {}",
                    shift, widget_base.id
                );
            }
            Some(Axis::Y) => {
                let mut y_base = y.base_mut();
                let widget_base = widget.base();

                // Move scrollbar
                y_base.layout.y =
                    (pos.y - self.cursor_offset).clamp(widget_base.layout.y, self.max_scroll_range);

                // Move container content
                let shift = (y_base.layout.y - widget_base.layout.y) * self.scroll_delta;
                for child in &widget.children {
                    child.base_mut().offset.y = -shift;
                }

                debug!(
                    "applying -{}px yshift offset to content for widget: {}",
                    shift, widget_base.id
                );
            }
            _ => unreachable!(),
        }
    }
    pub(crate) fn apply(
        &mut self,
        trigger: Rc<Trigger>,
        widget: &Container,
        e: Event<Signal>,
        last_cursor_pos: PhysicalPosition<f64>,
    ) {
        match e {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CursorMoved { position, .. } => {
                    if self.axis.is_some() {
                        self.on_scroll_movement(widget, position);
                        trigger.update();
                    } else {
                        self.on_cursor_movement(widget, position);
                    }
                }
                WindowEvent::MouseInput {
                    button: MouseButton::Left,
                    state: ElementState::Pressed,
                    ..
                } => {
                    self.on_pressed(widget, last_cursor_pos);

                    if self.axis.is_some() {
                        self.compute_scroll(widget);
                    }
                }
                WindowEvent::MouseInput {
                    button: MouseButton::Left,
                    state: ElementState::Released,
                    ..
                } => self.axis = None,
                _ => (),
            },
            _ => (),
        }
    }
}
