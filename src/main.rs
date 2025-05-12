use std::{cell::RefCell, rc::Rc};

use gemini::{
    action::{
        click::Click,
        hover::Hover,
        zoom::{Zoom, ZoomLevel},
        Action,
    },
    ui::{
        color::{Color, BLACK, BLUE, GREEN, RED},
        dom::DOM,
        widget::{canvas::Canvas, container::Container, BaseWidget, Widget as _},
    },
};
use log::info;
use winit::{event::MouseButton, window::Window};

struct Palette {
    selected: Color,
    palette: Vec<Color>,
}
impl Palette {
    fn new() -> Self {
        Self {
            selected: RED,
            palette: vec![RED, BLUE, BLACK, GREEN],
        }
    }
}

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).expect("Failed to init logger");
    info!("Starting demo UI app...");
    let palette = Click::new(Rc::new(RefCell::new(Palette::new())))
        .on(MouseButton::Left, |state, window, widget, event| {
            widget.style.color = state.borrow().selected.into();
            window.request_redraw();
        })
        .on(MouseButton::Right, |state, window, widget, event| {
            state.borrow_mut().selected = GREEN.into();
        });
    let cnv = Canvas::new()
        .set_width(256)
        .set_height(256)
        .set_grid(8, 1)
        .on_action(Action::ZoomInOut(Zoom::new_with_bounds(
            ZoomLevel::Zoom16x,
            2,
        )))
        .set_cells_actions(vec![Action::Click(Box::new(palette))]);

    let mut d = DOM::new(640, 512);
    d.add_widget(cnv);
    d.run();
}
