use log::debug;
use winit::{
    event::{Event, MouseScrollDelta, WindowEvent::MouseWheel},
    window::Window,
};

use crate::ui::{sync::Signal, widget::BaseWidget};

/// The UI zoom levels for user scaling
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub enum ZoomLevel {
    #[default]
    // Zoom at 2x scale
    Zoom2x,
    // Zoom at 4x scale
    Zoom4x,
    // Zoom at 8x scale
    Zoom8x,
    // Zoom at 16x scale
    Zoom16x,
}
impl From<ZoomLevel> for f64 {
    fn from(value: ZoomLevel) -> Self {
        match value {
            ZoomLevel::Zoom2x => 2.0,
            ZoomLevel::Zoom4x => 4.0,
            ZoomLevel::Zoom8x => 8.0,
            ZoomLevel::Zoom16x => 16.0,
        }
    }
}

/// The `Zoom` struct allows widgets to have the ability
/// to respond to zoom events
///
/// Default:
///
/// - A zoom level of 2x with no upper or lower restrictions
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Zoom {
    /// The scaling multiple for zooming in/out of a widget
    ///
    /// Typically the number should be a multiple of your width and height
    scale: ZoomLevel,
    /// The amount of steps in/out a zoom action is bounded to
    steps: Option<u32>,
    lower_upper: Option<(f64, f64, f64, f64)>,
}
impl Zoom {
    /// Create a new `Zoom` action
    ///
    /// The `scale` provides a scaling multiple for zooming in/out of a widget
    pub fn new(scale: ZoomLevel) -> Self {
        Self {
            scale,
            ..Default::default()
        }
    }
    /// Create a new `Zoom` action with a lower and upper level bound
    /// based on `steps` allowed from original image scale
    pub fn new_with_bounds(scale: ZoomLevel, steps: u32) -> Self {
        Self {
            scale,
            steps: Some(steps),
            ..Default::default()
        }
    }
    pub(crate) fn apply(&mut self, window: &Window, widget: &mut BaseWidget, event: Event<Signal>) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                MouseWheel { delta, .. } => match delta {
                    MouseScrollDelta::LineDelta(_, y) => {
                        debug!("triggered zoom in/out for widget: {}", widget.id);

                        let scale: f64 = self.scale.into();

                        // Apply scaling factor
                        let scaled_w = widget.layout.w + (y as f64 * scale);
                        let scaled_h = widget.layout.h + (y as f64 * scale);

                        // Create bounds of zooming in and out if applicable (ONLY ONCE)
                        if self.lower_upper.is_none() {
                            if let Some(steps) = &self.steps {
                                let steps = (scale as u32 * steps) as f64;

                                let (min_h, max_h, min_w, max_w) = (
                                    (widget.layout.h - steps).abs(),
                                    (widget.layout.h + steps)
                                        .min(window.inner_size().height as f64),
                                    (widget.layout.w - steps).abs(),
                                    (widget.layout.w + steps).min(window.inner_size().width as f64),
                                );
                                self.lower_upper = Some((min_h, max_h, min_w, max_w));

                                debug!("zoom in/out bounds created for widget:  {} - MIN_WIDTH: {} MAX_WIDTH: {} MIN_HEIGHT: {} MAX_HEIGHT: {}", widget.id, min_w, max_w, min_h, max_h);
                            }
                        }

                        let (final_scaled_h, final_scaled_w) =
                            if let Some(bounds) = self.lower_upper {
                                let (min_h, max_h, min_w, max_w) = bounds;
                                (scaled_h.clamp(min_h, max_h), scaled_w.clamp(min_w, max_w))
                            } else {
                                (scaled_h, scaled_w)
                            };

                        widget.layout.w = final_scaled_w;
                        widget.layout.h = final_scaled_h;

                        window.request_redraw();
                    }
                    _ => unreachable!(),
                },
                _ => (),
            },
            _ => (),
        }
    }
}
