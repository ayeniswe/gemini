use std::{cell::RefCell, rc::Rc};

use gemini::{
    action::{
        click::{Click, ClickHandler},
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
use winit::window::Window;

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
impl ClickHandler for Palette {
    fn apply(&mut self, window: &Window, widget: &mut BaseWidget) {
        widget.style.color.set_color(self.selected);
        window.request_redraw();
    }
}

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).expect("Failed to init logger");
    info!("Starting demo UI app...");

    let cnv = Canvas::new()
        .set_width(256)
        .set_height(256)
        .set_grid(8, 1)
        .on_action(Action::ZoomInOut(Zoom::new_with_bounds(
            ZoomLevel::Zoom16x,
            2,
        )))
        .set_cells_actions(vec![
            Action::LeftClick(Click::new(Rc::new(RefCell::new(Palette::new())))),
            Action::Hover(Hover::new(Color::RGBA(235, 235, 235, 75))),
        ]);

    let mut d = DOM::new(640, 512);
    d.add_widget(cnv);
    d.run();
}
