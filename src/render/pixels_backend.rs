use pixels::Pixels;
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Rect, Transform};

use crate::render::Renderer;
use crate::ui::canvas::Canvas;
use crate::ui::color::Color;
use crate::ui::{Hoverable, Widget};

pub struct PixelsRenderer {
    pixels: Pixels,
}
impl PixelsRenderer {
    pub fn new(pixels: Pixels) -> Self {
        Self { pixels }
    }
    /// Returns either black or white based on the perceived brightness of a background color.
    ///
    /// This function calculates the luminance of the given `bg` color using the
    /// standard formula for relative luminance:
    /// `luminance = 0.299 * R + 0.587 * G + 0.114 * B`
    ///
    fn get_contrast_color(bg: Color) -> Color {
        let (r, g, b) = bg.into();
        // Detect luminance
        if 0.299 * (r as f32 / 255.0) + 0.587 * (g as f32 / 255.0) + 0.114 * (b as f32 / 255.0)
            > 0.5
        {
            Color::RGBA(0, 0, 0, 255) // Use black
        } else {
            Color::RGBA(255, 255, 255, 255) // Use white
        }
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
        paint.set_color((*color).into());
        pixmap.fill_path(
            &path,
            &paint,
            FillRule::Winding,
            Transform::identity(),
            None,
        );

        pixmap
    }
    /// Copies the pixel data from the given `Pixmap` onto the current frame buffer.
    ///
    /// This method performs a direct memory copy (blit) from the source `Pixmap`
    /// to the destination frame managed by the `pixels` instance. It assumes both
    /// the source and destination have the same pixel format (e.g., RGBA, 4 bytes per pixel)
    /// and that the destination frame is large enough to accommodate the pixmap.
    fn blit_on(&mut self, offset_x: u32, offset_y: u32, map: &Pixmap) {
        let frame_width = self.pixels.texture().width();
        let frame = self.pixels.frame_mut();
        for y in 0..map.height() {
            for x in 0..map.width() {
                // Row major layout follows this formula
                let map_idx = ((y * map.width() + x) * 4) as usize;
                let frame_idx = (((y + offset_y) * frame_width + (x + offset_x)) * 4) as usize;
                if frame_idx + 3 < frame.len() {
                    frame[frame_idx..frame_idx + 4]
                        .copy_from_slice(&map.data()[map_idx..map_idx + 4]);
                }
            }
        }
    }
    fn draw_line(w: u32, h: u32, color: &Color) -> Pixmap {
        let mut pixmap = Pixmap::new(w, h).unwrap();
        let mut paint = Paint::default();
        paint.set_color((*color).into());
        pixmap.fill_rect(
            Rect::from_xywh(0.0, 0.0, w as f32, h as f32).unwrap(),
            &paint,
            tiny_skia::Transform::identity(),
            None,
        );

        pixmap
    }
    fn draw_gridlines(
        &mut self,
        pos: (u32, u32),
        width: u32,
        height: u32,
        spacing: u32,
        color: Color,
        thickness: u32,
    ) {
        let (x, y) = pos;

        let h_lines_spacing = height / spacing;
        let w_lines_spacing = width / spacing;
        // Draw column gridlines
        for col in 1..spacing as usize {
            let spacing = w_lines_spacing * col as u32;
            let line = PixelsRenderer::draw_line(
                thickness,
                height,
                &PixelsRenderer::get_contrast_color(color),
            );
            self.blit_on(x + spacing, y, &line);
        }
        // Draw row gridlines
        for row in 1..spacing as usize {
            let spacing = h_lines_spacing * row as u32;
            let line = PixelsRenderer::draw_line(
                width,
                thickness,
                &PixelsRenderer::get_contrast_color(color),
            );
            self.blit_on(x, y + spacing, &line);
        }
    }
    /// Determines the appropriate color to use when rendering a widget based on its hover state.
    ///
    /// If the widget is currently hovered and has a custom hover color defined,
    /// that color is returned. Otherwise, it falls back to the widget's default color.
    fn get_hover_color<T: Hoverable>(widget: &T) -> Color {
        // Display color
        if widget.hovered() {
            // Display color on hover (if applicable)
            if let Some(color) = &widget.hover_color() {
                *color
            } else {
                *widget.color()
            }
        } else {
            *widget.color()
        }
    }
}
impl Renderer for PixelsRenderer {
    fn clear(&mut self) {
        let frame = self.pixels.frame_mut();
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0, 0, 0, 255]);
        }
    }
    fn draw_canvas(&mut self, canvas: &Canvas) {
        let color = PixelsRenderer::get_hover_color(canvas);

        self.draw_widget(canvas, color);

        if let Some(grid) = &canvas.grid {
            self.draw_gridlines(
                canvas.pos(),
                canvas.width,
                canvas.height,
                grid.spacing,
                color,
                1,
            );
        }
    }
    fn draw_widget<T: Widget>(&mut self, widget: &T, color: Color) {
        let frame_width = self.pixels.texture().width();
        let frame = self.pixels.frame_mut();

        // Draw widget
        let (widget_x, widget_y) = widget.pos();
        if widget.radius() > 0 {
            // Offshoot to skia for smooth draws (if needed)
            let rounded_rect = PixelsRenderer::draw_rounded_rect(
                widget_x as f32,
                widget_y as f32,
                widget.width() as f32,
                widget.height() as f32,
                widget.radius() as f32,
                &color,
            );
            self.blit_on(widget_x, widget_y, &rounded_rect);
        } else {
            let color: [u8; 4] = color.into();
            for y in widget_y..widget_y + widget.height() {
                for x in widget_x..widget_x + widget.width() {
                    // Row major layout follows this formula
                    let idx = ((y * frame_width + x) * 4) as usize;
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
