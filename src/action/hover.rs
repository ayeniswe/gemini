use log::debug;
use winit::{
    event::{Event, WindowEvent},
    window::Window,
};

use crate::ui::{color::Color, widget::BaseWidget};

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hover {
    pub hover_color: Color,
    pub base_color: Color,
    pub hovered: bool,
}
impl Hover {
    pub fn new(color: Color) -> Self {
        Self {
            hover_color: color,
            ..Default::default()
        }
    }
    fn update_hover_state(&mut self, widget: &mut BaseWidget, mx: f64, my: f64) {
        self.hovered = widget.layout.is_inbounds(mx, my)
    }
    pub(crate) fn apply(&mut self, window: &Window, widget: &mut BaseWidget, event: Event<()>) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CursorMoved { position, .. } => {
                    let previous_hover_state = self.hovered;

                    self.update_hover_state(widget, position.x, position.y);

                    if previous_hover_state != self.hovered {
                        if self.hovered {
                            debug!("triggered hover for widget: {}", widget.id);

                            // Swap palette..we can always expect the base to be retrieved
                            // since swap will never happen unless an intial hover took place
                            self.base_color = widget.style.color;
                            widget.style.color = self.hover_color;
                        } else {
                            widget.style.color = self.base_color;
                        }

                        window.request_redraw();
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}
