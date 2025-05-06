use winit::{
    event::{Event, MouseScrollDelta, WindowEvent::MouseWheel},
    window::Window,
};

use crate::ui::widget::BaseWidget;

use super::Actionable;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Zoom {
    scale: f32,
}
impl Zoom {
    pub fn new(scale: f32) -> Self {
        Self { scale }
    }
}
impl Actionable for Zoom {
    fn apply_action(&mut self, _: &Window, widget: &mut BaseWidget, event: Event<()>) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                MouseWheel { delta, .. } => match delta {
                    MouseScrollDelta::LineDelta(_, y) => {
                        widget.layout.w = (widget.layout.w as f32 + (y * self.scale)) as u32;
                        widget.layout.h = (widget.layout.h as f32 + (y * self.scale)) as u32;
                    }
                    _ => unreachable!(),
                },
                _ => (),
            },
            _ => (),
        }
    }
}
