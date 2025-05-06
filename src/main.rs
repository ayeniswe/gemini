use gemini::{
    render::{pixels_backend::PixelsRenderer, Renderer},
    ui::{
        canvas::Canvas,
        color::{Color, BLACK, RED},
        Hoverable as _, Widget,
    },
};
use log::info;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::{LogicalSize, PhysicalPosition},
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).expect("Failed to init logger");
    info!("Starting demo UI app...");

    // === Window + Pixels Init ===
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Gemini - UI Framework")
        .with_inner_size(LogicalSize::new(640, 320))
        .build(&event_loop)
        .unwrap();

    let size = window.inner_size();
    let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
    let pixels = Pixels::new(size.width, size.height, surface_texture).unwrap();

    // === Create Renderer ===
    let mut renderer = PixelsRenderer::new(pixels);

    // === Create Canvas ===
    let mut cnv = Canvas::new(256, 256);
    let cnv = cnv.set_hover_color(RED).set_gridlines(8);

    // === Event Loop ===
    let mut cursor_position = PhysicalPosition::new(0.0, 0.0);
    event_loop
        .run(|event, target| {
            match event {
                // Event::NewEvents(start_cause) => todo!(),
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CloseRequested => target.exit(),
                        WindowEvent::RedrawRequested => {
                            renderer.clear();

                            renderer.draw_canvas(&cnv);

                            renderer.present();
                        }
                        // WindowEvent::ActivationTokenDone { serial, token } => todo!(),
                        // WindowEvent::Moved(physical_position) => todo!(),
                        // WindowEvent::Resized(physical_size) => todo!(),
                        // WindowEvent::Destroyed => todo!(),
                        // WindowEvent::DroppedFile(path_buf) => todo!(),
                        // WindowEvent::HoveredFile(path_buf) => todo!(),
                        // WindowEvent::HoveredFileCancelled => todo!(),
                        // WindowEvent::Focused(_) => todo!(),
                        // WindowEvent::KeyboardInput { device_id, event, is_synthetic } => todo!(),
                        // WindowEvent::ModifiersChanged(modifiers) => todo!(),
                        // WindowEvent::Ime(ime) => todo!(),
                        WindowEvent::CursorMoved { position, .. } => {
                            cursor_position = position;

                            let mut redraw_needed = false;

                            let previous_hover_state = cnv.hovered;

                            cnv.update_hover_state(position.x as u32, position.y as u32);

                            if previous_hover_state != cnv.hovered {
                                redraw_needed = true;
                            }

                            if redraw_needed {
                                window.request_redraw();
                            }
                        }
                        // WindowEvent::CursorEntered { device_id } => todo!(),
                        // WindowEvent::CursorLeft { device_id } => todo!(),
                        // WindowEvent::MouseWheel { device_id, delta, phase } => todo!(),
                        // WindowEvent::MouseInput { device_id, state, button } => todo!(),
                        // WindowEvent::TouchpadMagnify { device_id, delta, phase } => todo!(),
                        // WindowEvent::SmartMagnify { device_id } => todo!(),
                        // WindowEvent::TouchpadRotate { device_id, delta, phase } => todo!(),
                        // WindowEvent::TouchpadPressure { device_id, pressure, stage } => todo!(),
                        // WindowEvent::AxisMotion { device_id, axis, value } => todo!(),
                        // WindowEvent::Touch(touch) => todo!(),
                        // WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer } => todo!(),
                        // WindowEvent::ThemeChanged(theme) => todo!(),
                        // WindowEvent::Occluded(_) => todo!(),
                        _ => (),
                    }
                }
                // Event::AboutToWait => {}
                // Event::DeviceEvent { device_id, event } => todo!(),
                // Event::UserEvent(_) => todo!(),
                // Event::Suspended => todo!(),
                // Event::Resumed => todo!(),
                // Event::LoopExiting => todo!(),
                // Event::MemoryWarning => todo!(),
                _ => (),
            }
        })
        .unwrap();
}
