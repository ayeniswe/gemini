use std::{cell::RefCell, rc::Rc};

use gemini::{
    action::{
        click::Click,
        hover::Hover,
        zoom::{Zoom, ZoomLevel},
        Action,
    },
    ui::{
        color::{Color, BLACK, BLUE, GREEN, LIGHT_GRAY, RED, WHITE},
        dom::DOM,
        widget::{canvas::Canvas, container::Container, heading::Heading, Widget},
    },
};
use log::info;
use winit::event::MouseButton;

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
        .on(MouseButton::Left, |state, trigger, widget, event| {
            widget.style.color = state.borrow().selected.into();
            trigger.update();
        })
        .on(MouseButton::Right, |state, trigger, widget, event| {
            state.borrow_mut().selected = GREEN.into();
        });

    let cnv = Canvas::new()
        .set_width(512.0)
        .set_height(512.0)
        .set_color(WHITE)
        .set_grid(8, 1.0, WHITE)
        .set_cells_actions(vec![
            Action::Hover(Hover::new(LIGHT_GRAY)),
            Action::Click(Box::new(palette)),
        ]);

    let mut d = DOM::new(640, 512);
    d.add_widget(cnv);
    d.run();
}
