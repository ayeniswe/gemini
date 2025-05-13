use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crypto::verify_checksum;
use gemini::{
    action::{
        click::Click,
        hover::Hover,
        zoom::{Zoom, ZoomLevel},
        Action,
    },
    ui::{
        color::{Color, BLACK, BLUE, GREEN, RED, WHITE},
        dom::DOM,
        layout::FlexLayout,
        sync::{Emitter, Trigger},
        widget::{canvas::Canvas, container::Container, BaseWidget, Widget as _},
    },
};
use hex::decode;
use log::info;
use openssl::hash::hash;
use openssl::hash::{self, MessageDigest};
use parse::System;
use rand::{seq::SliceRandom as _, thread_rng};
use winit::{event::MouseButton, event_loop::EventLoopProxy, window::Window};

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

pub mod crypto;
pub mod parse;

// fn main() {
//     log4rs::init_file("log4rs.yaml", Default::default()).expect("Failed to init logger");
//     info!("Starting demo UI app...");
//     let palette = Click::new(Rc::new(RefCell::new(Palette::new())))
//         .on(MouseButton::Left, |state, window, widget, event| {
//             widget.style.color = state.borrow().selected.into();
//             window.request_redraw();
//         })
//         .on(MouseButton::Right, |state, window, widget, event| {
//             state.borrow_mut().selected = GREEN.into();
//         });
//     let cnv = Canvas::new()
//         .set_width(256)
//         .set_height(256)
//         .set_label("good")
//         .set_label_horizontal()
//         .set_label_vertical()
//         .set_grid(8, 1)
//         .set_grid_range((8, 8), 1)
//         .on_action(Action::ZoomInOut(Zoom::new_with_bounds(
//             ZoomLevel::Zoom16x,
//             2,
//         )))
//         .set_cells_actions(vec![Action::Click(Box::new(palette))]);

//     let mut d = DOM::new(640, 512);
//     d.add_widget(cnv);
//     d.run();
// }
struct SystemIndex {
    color: Color,
}
impl SystemIndex {
    pub fn new() -> Self {
        Self { color: RED }
    }
}
impl Emitter for SystemIndex {
    fn run(&self, trigger: Trigger) {
        let choices = vec![GREEN, BLACK, BLUE];

        loop {
            let mut rng = thread_rng();
            let color = *choices.choose(&mut rng).unwrap();

            trigger.trigger_update(move |widget| {
                widget.style.color.set_color(color);
            });

            thread::sleep(Duration::from_secs(1))
        }
    }
}

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).expect("Failed to init logger");
    info!("Starting demo UI app...");

    // Load GCS default baseline
    // User can dynamically change this but
    // needs an initial point to start
    let file = File::open("config/baseline-default.json").unwrap();
    let systems: HashMap<String, System> = serde_json::from_reader(BufReader::new(file)).unwrap();

    // Initalizes gui to display all widgets
    // and handle interactions/event handling
    let mut dom = DOM::new(640, 320);

    // Dynamically set baseline based on user configuration
    let mut central_panel = Container::new()
        .set_width(640)
        .set_height(320)
        .set_horizontal()
        .set_vertical()
        .set_gap(5)
        .set_flex_layout(FlexLayout::FlexGrid(4))
        .set_color(RED);

    for system in systems.iter().enumerate() {
        let (idx, (name, data)) = system;
        central_panel.add_widget(
            Container::new()
                .set_width(100)
                .set_height(50)
                .set_label(&name)
                .set_label_size(12)
                .set_label_horizontal()
                .set_label_vertical()
                .connect(SystemIndex::new())
                .set_color(WHITE),
        );
    }
    dom.add_widget(central_panel);

    // Should be last command
    // Acts as the final entrypoint of entire GUI application
    dom.run();
}
