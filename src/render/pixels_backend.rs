use pixels::Pixels;
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};

use crate::render::Renderer;
use crate::ui::color::Color;
use crate::ui::Widget;

pub struct PixelsRenderer {
    pixels: Pixels,
}
impl PixelsRenderer {
    pub fn new(pixels: Pixels) -> Self {
        Self { pixels }
    }
    fn draw_rounded_rect(x: f32, y: f32, w: f32, h: f32, r: f32, color: &Color) -> Pixmap {
        // Since the radius is created using contour we need to buffer some space for the map to
        // be correctly blit later and account for rgba with 4bytes of room
        let mut pixmap = Pixmap::new((w + (r * 4.0)) as u32, (h + (r * 4.0)) as u32).unwrap();

        // Anti aliased a rounded rect
        let mut pb = PathBuilder::new();
        // Start at top-left corner, move to start of top edge
        pb.move_to(x + r, y);
        // Top edge
        pb.line_to(x + w - r, y);
        // Top-right corner
        pb.quad_to(x + w, y, x + w, y + r);
        // Right edge
        pb.line_to(x + w, y + h - r);
        // Bottom-right corner
        pb.quad_to(x + w, y + h, x + w - r, y + h);
        // Bottom edge
        pb.line_to(x + r, y + h);
        // Bottom-left corner
        pb.quad_to(x, y + h, x, y + h - r);
        // Left edge
        pb.line_to(x, y + r);
        // Top-left corner
        pb.quad_to(x, y, x + r, y);
        pb.close();
        let path = pb.finish().unwrap();

        // Map to blit to main buffer
        let mut paint = Paint::default();
        let color = match color {
            Color::RGBA(r, g, b, a) => tiny_skia::Color::from_rgba8(*r, *g, *b, *a),
        };
        paint.set_color(color);
        pixmap.fill_path(
            &path,
            &paint,
            FillRule::Winding,
            Transform::identity(),
            None,
        );

        pixmap
    }
}
impl Renderer for PixelsRenderer {
    fn clear(&mut self) {
        let frame = self.pixels.frame_mut();
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0, 0, 0, 255]);
        }
    }
    fn draw_widget<T: Widget>(&mut self, widget: &T) {
        let frame_width = self.pixels.texture().width();
        let frame = self.pixels.frame_mut();

        // Display color
        let color = if widget.hovered() {
            // Display color on hover (if applicable)
            if let Some(color) = &widget.hover_color() {
                color
            } else {
                &widget.color()
            }
        } else {
            &widget.color()
        };

        // Draw rect
        let (widget_x, widget_y) = widget.pos();
        if widget.radius() > 0 {
            // Offshoot to skia for smooth draws (if needed)
            let rounded_rect = PixelsRenderer::draw_rounded_rect(
                widget_x as f32,
                widget_x as f32,
                widget.width() as f32,
                widget.height() as f32,
                widget.radius() as f32,
                &color,
            );
            // Blit to main buffer
            for y in 0..rounded_rect.height() {
                for x in 0..rounded_rect.width() {
                    // Row major layout follows this formula
                    let rounded_rect_idx = ((y * rounded_rect.width() + x) * 4) as usize;
                    let frame_idx = (((y) * frame_width + (x)) * 4) as usize;
                    frame[frame_idx..frame_idx + 4].copy_from_slice(
                        &rounded_rect.data()[rounded_rect_idx..rounded_rect_idx + 4],
                    );
                }
            }
        } else {
            for y in widget_y..widget_y + widget.height() {
                for x in widget_x..widget_x + widget.width() {
                    // Row major layout follows this formula
                    let idx = ((y * frame_width + x) * 4) as usize;
                    let color: [u8; 4] = (*color).into();
                    frame[idx..idx + 4].copy_from_slice(&color);
                }
            }
        }

        // (Optional) draw text later with `ab_glyph` or another crate
    }
    fn present(&mut self) {
        self.pixels.render().unwrap();
    }
}
