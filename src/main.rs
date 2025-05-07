use gemini::{
    action::{zoom::Zoom, Action},
    ui::{
        widget::{canvas::Canvas, Widget as _},
        DOM,
    },
};
use log::info;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).expect("Failed to init logger");
    info!("Starting demo UI app...");

    // === Create Canvas ===
    let cnv = Canvas::new();
    cnv.set_width(64)
        .set_height(64)
        .set_gridlines(8)
        .on_action(Action::ZoomInOut(Zoom::new(16.0)));

    let mut d = DOM::<Canvas>::new(640, 320);
    d.add_widget(cnv);
    d.run();
}
