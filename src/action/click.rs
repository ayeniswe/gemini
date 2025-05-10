use std::{cell::RefCell, rc::Rc};

use log::debug;
use winit::{
    event::{Event, MouseButton, WindowEvent},
    window::Window,
};

use crate::ui::widget::BaseWidget;

/// The `Click` struct allows widgets to have the ability
/// to respond to any mouse click event
#[derive(Clone)]
pub struct Click {
    selected: bool,
    handler: Rc<RefCell<dyn ClickHandler>>,
}
impl Click {
    pub fn new(handler: Rc<RefCell<dyn ClickHandler>>) -> Self {
        Self {
            handler,
            selected: false,
        }
    }
    fn update_selection(&mut self, widget: &mut BaseWidget, mx: f64, my: f64) {
        self.selected = widget.layout.is_inbounds(mx, my)
    }
    pub(crate) fn apply(
        &mut self,
        btn: MouseButton,
        window: &Window,
        widget: &mut BaseWidget,
        event: Event<()>,
    ) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CursorMoved { position, .. } => {
                    self.update_selection(widget, position.x, position.y);
                }
                WindowEvent::MouseInput { button, state, .. } => {
                    if btn == button && self.selected  && state.is_pressed(){
                        match button {
                            winit::event::MouseButton::Left => {                                
                                debug!("triggered left click for widget: {}", widget.id);
                                self.handler.borrow_mut().apply(window, widget)
                            }
                            winit::event::MouseButton::Right => todo!(),
                            winit::event::MouseButton::Middle => todo!(),
                            winit::event::MouseButton::Back => todo!(),
                            winit::event::MouseButton::Forward => todo!(),
                            winit::event::MouseButton::Other(_) => todo!(),
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}
/// The trait `ClickHandler` provides a
/// extensible way for users to hijack events
/// for a particular widget and do whatever
/// their hearts desire :)
pub trait ClickHandler {
    fn apply(&mut self, window: &Window, widget: &mut BaseWidget);
}
