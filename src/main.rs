use gemini::{
    action::{
        hover::Hover,
        zoom::{Zoom, ZoomLevel},
        Action,
    },
    ui::{
        color::Color,
        dom::DOM,
        widget::{canvas::Canvas, Widget as _},
    },
};
use log::info;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).expect("Failed to init logger");
    info!("Starting demo UI app...");

    // === Create Canvas ===
    let cnv = Canvas::new()
        .set_width(256)
        .set_height(256)
        .set_label("The main man is here")
        .set_grid(8, 1)
        .on_action(Action::ZoomInOut(Zoom::new_with_bounds(
            ZoomLevel::Zoom16x,
            2,
        )))
        .set_cells_actions(vec![Action::Hover(Hover::new(Color::RGBA(
            235, 235, 235, 255,
        )))]);

    let mut d = DOM::new(640, 320);
    d.add_widget(cnv);
    d.run();
}
