use std::rc::Rc;

use log::debug;
use winit::{
    event::{Event, WindowEvent},
    window::Window,
};

use crate::ui::{
    color::{Color, ColorMode},
    sync::{Signal, Trigger},
    widget::BaseWidget,
};

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hover {
    pub hover_color: Color,
}
impl Hover {
    pub fn new(color: Color) -> Self {
        Self {
            hover_color: color,
            ..Default::default()
        }
    }
    pub(crate) fn apply(
        &mut self,
        trigger: Rc<Trigger>,
        widget: &mut BaseWidget,
        event: Event<Signal>,
    ) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CursorMoved { position, .. } => {
                    let previous_hover_state = widget.state.hovered;

                    widget.state.hovered = widget.layout.is_inbounds(position.x, position.y);

                    if previous_hover_state != widget.state.hovered {
                        if widget.state.hovered {
                            debug!("triggered hover for widget: {}", widget.id);
                            widget
                                .style
                                .color
                                .set_mode(ColorMode::Overlay(self.hover_color));
                        } else {
                            debug!("triggered unhover for widget: {}", widget.id);
                            widget.style.color.set_mode(ColorMode::Solid);
                        }

                        trigger.update()
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}
