use std::rc::Rc;

use ab_glyph::{point, Font as _, FontRef, Glyph, PxScale, ScaleFont as _};
use pixels::Pixels;
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Rect, Transform};

use crate::{
    render::Renderer,
    ui::{
        color::{Color, BLACK, TRANSPARENT, WHITE},
        layout::{Layout, Point},
        text::DEFAULT_FONT,
        widget::{canvas::Canvas, container::Container, Widget, WidgetI},
    },
};

use super::row_major;

type NoCustom = Option<fn(&mut PixelsRenderer)>;
const NO_CUSTOM: NoCustom = None;

pub(crate) struct PixelsRenderer {
    pixels: Pixels,
}
impl PixelsRenderer {
    pub(crate) fn new(pixels: Pixels) -> Self {
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
            BLACK
        } else {
            WHITE
        }
    }
    /// Copies the pixel data from the given `Pixmap` onto the current frame buffer.
    ///
    /// This method performs a direct memory copy (blit) from the source `Pixmap`
    /// to the destination frame managed by the `pixels` instance. It assumes both
    /// the source and destination have the same pixel format (e.g., RGBA, 4 bytes per pixel)
    /// and that the destination frame is large enough to accommodate the pixmap.
    fn blit_on(
        &mut self,
        offset_x: i32,
        offset_y: i32,
        map: &Pixmap,
        clipping_region: Option<Layout>,
    ) {
        let frame_width = self.pixels.texture().width();
        let frame = self.pixels.frame_mut();
        let map_buffer = map.data();

        for y in 0..map.height() {
            for x in 0..map.width() {
                // Ignore drawing pixels off screen
                let x_normalized = x as i32 + offset_x;
                let y_normalized = y as i32 + offset_y;
                if x_normalized < 0 || y_normalized < 0 {
                    continue;
                }

                // Ignore drawing pixels that fall outside Container range
                if let Some(clipping) = clipping_region {
                    if (x_normalized > clipping.w as i32 || x_normalized < clipping.x as i32)
                        || y_normalized > clipping.h as i32
                        || y_normalized < clipping.y as i32
                    {
                        continue;
                    }
                }

                let frame_idx = row_major(x_normalized as u32, y_normalized as u32, frame_width);
                let map_idx = row_major(x, y, map.width());
                if frame_idx + 3 < frame.len() {
                    let out = &Color::src_over_blend(
                        &map_buffer[map_idx..map_idx + 4],
                        &frame[frame_idx..frame_idx + 4],
                    );
                    frame[frame_idx..frame_idx + 4].copy_from_slice(out);
                }
            }
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
    /// # Note
    ///
    /// Round all floats to nearest
    fn draw_line(w: f64, h: f64, color: &Color) -> Pixmap {
        // We can not render anything lower than zero
        // since nothing will show...duhhh so we limit it to 1 minimal
        let map_width = (w.round() as u32).max(1);
        let map_height = (h.round() as u32).max(1);
        let mut pixmap = Pixmap::new(map_width, map_height).unwrap();
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
    /// # Note
    ///
    /// Round all floats to nearest
    fn draw_gridlines(
        &mut self,
        pos: (f64, f64),
        width: f64,
        height: f64,
        spacing: Point,
        color: Color,
        thickness: f64,
    ) {
        let (x, y) = pos;

        let h_lines_spacing = height / spacing.y;
        let w_lines_spacing = width / spacing.x;
        // Draw column gridlines
        for col in 1..spacing.x as usize {
            let spacing = w_lines_spacing * col as f64;
            let line = PixelsRenderer::draw_line(
                thickness,
                height,
                &PixelsRenderer::get_contrast_color(color),
            );
            self.blit_on((x + spacing).round() as i32, y.round() as i32, &line, None);
        }
        // Draw row gridlines
        for row in 1..spacing.y as usize {
            let spacing = h_lines_spacing * row as f64;
            let line = PixelsRenderer::draw_line(
                width,
                thickness,
                &PixelsRenderer::get_contrast_color(color),
            );
            self.blit_on(x.round() as i32, (y + spacing).round() as i32, &line, None);
        }
    }
    fn draw_text(text: &str, font_size: f32, color: Color) -> Pixmap {
        // Load font face with scale
        let font = FontRef::try_from_slice(DEFAULT_FONT).unwrap();
        let scale = PxScale::from(font_size);
        let font_scaled = font.as_scaled(scale);

        // We need the respective glyphs to know how to cutout our character
        // styling (what it will look like)
        let mut glyphs: Vec<Glyph> = Vec::new();
        let mut caret = point(0.0, font_scaled.ascent());
        for c in text.chars() {
            let glyph = font_scaled
                .glyph_id(c)
                .with_scale_and_position(scale, caret);
            let id = glyph.id;

            glyphs.push(glyph);

            // Move over for next character coming
            // as of now we support only horizontal text
            caret.x += font_scaled.h_advance(id);
        }

        // We now have the expected total width and lenght to buffer these
        // pixels of each char in text
        // Double height is needed for possible descent chars and
        // could be done better but as of now this is fine
        let text_height = (font_scaled.ascent() - font_scaled.descent()).ceil();
        let mut pixmap = Pixmap::new(caret.x.ceil() as u32, text_height as u32).unwrap();
        let pixmap_buffer_width = pixmap.width();
        let pixmap_buffer = pixmap.data_mut();

        let color: [u8; 4] = color.into();
        for glyph in glyphs {
            // Get outline of text so we can draw within
            // bounds since all glyphs can be classified as
            // as bounding box thats cutout
            if let Some(outline) = font.outline_glyph(glyph) {
                let bounds = outline.px_bounds();

                // Now we know the points to draw
                outline.draw(|x, y, c| {
                    let x = x as u32 + bounds.min.x as u32;
                    let y = y as u32 + bounds.min.y as u32;

                    let idx = row_major(x, y, pixmap_buffer_width);
                    if idx + 3 < pixmap_buffer.len() {
                        pixmap_buffer[idx] = (color[0] as f32) as u8;
                        pixmap_buffer[idx + 1] = (color[1] as f32) as u8;
                        pixmap_buffer[idx + 2] = (color[2] as f32) as u8;
                        // The c value is coverage multiplier to smooth out
                        // drawing
                        pixmap_buffer[idx + 3] =
                            (color[3] as f32 * c).round().clamp(0.0, 255.0) as u8;
                    }
                });
            }
        }
        pixmap
    }
    fn draw_canvas(&mut self, widget: &Canvas, clipping_region: Option<Layout>) {
        if let Some(grid) = &mut *widget.grid.borrow_mut() {
            self.draw_widget(
                widget,
                Some(|renderer: &mut PixelsRenderer| {
                    let widget = widget.base();

                    // Draw gridlines
                    renderer.draw_gridlines(
                        (
                            widget.offset.x + widget.layout.x,
                            widget.offset.y + widget.layout.y,
                        ),
                        widget.layout.w,
                        widget.layout.h,
                        grid.size,
                        widget.style.color.into(),
                        grid.thickness,
                    );

                    grid.on_cell(|_, c| {
                        renderer.draw_widget(c.as_ref(), NO_CUSTOM, clipping_region);
                    });
                }),
                clipping_region,
            );
        } else {
            self.draw_widget(widget, NO_CUSTOM, clipping_region);
        }
    }
    /// # Note
    ///
    /// Round all floats to nearest
    fn draw_widget<F: Fn(&mut Self)>(
        &mut self,
        widget: &dyn Widget,
        custom_render: Option<F>,
        clipping_region: Option<Layout>,
    ) {
        let widget_base = widget.base();

        let color = widget_base.style.color.into();

        // Draw widget base with constraints
        if widget_base.style.radius > 0 {
            // Offshoot to skia for smooth draws (if needed)
            let rounded_rect = PixelsRenderer::draw_rounded_rect(
                (widget_base.offset.x + widget_base.layout.x) as f32,
                (widget_base.offset.y + widget_base.layout.y) as f32,
                widget_base.layout.w as f32,
                widget_base.layout.h as f32,
                widget_base.style.radius as f32,
                &color,
            );

            self.blit_on(
                (widget_base.offset.x + widget_base.layout.x).round() as i32,
                (widget_base.offset.y + widget_base.layout.y).round() as i32,
                &rounded_rect,
                clipping_region,
            );
        }

        let frame_width = self.pixels.texture().width();
        let frame = self.pixels.frame_mut();

        // Draw normal widget base
        if widget_base.style.radius == 0 {
            let color: [u8; 4] = color.into();
            for y in (widget_base.offset.y + widget_base.layout.y) as i32
                ..(widget_base.offset.y + widget_base.layout.y + widget_base.layout.h).round()
                    as i32
            {
                for x in (widget_base.offset.x + widget_base.layout.x) as i32
                    ..(widget_base.offset.x + widget_base.layout.x + widget_base.layout.w).round()
                        as i32
                {
                    // Ignore drawing pixels off screen
                    if x < 0 || y < 0 {
                        continue;
                    }

                    // Ignore drawing pixels that fall outside Container range
                    if let Some(clipping) = clipping_region {
                        if (x > clipping.w as i32 || x < clipping.x as i32)
                            || y > clipping.h as i32
                            || y < clipping.y as i32
                        {
                            continue;
                        }
                    }

                    // Row major layout follows this formula
                    let idx = row_major(x as u32, y as u32, frame_width);
                    if idx + 3 < frame.len() {
                        frame[idx..idx + 4].copy_from_slice(&color);
                    }
                }
            }
        }

        if let Some(render) = custom_render {
            render(self);
        }

        // Draw text
        if !widget_base.text.label.is_empty() {
            let text = PixelsRenderer::draw_text(
                &widget_base.text.label,
                widget_base.text.font_size as f32,
                BLACK,
            );
            self.blit_on(
                (widget_base.offset.x + widget_base.layout.x + widget_base.text.pos.x).round()
                    as i32,
                (widget_base.offset.y + widget_base.layout.y + widget_base.text.pos.y).round()
                    as i32,
                &text,
                clipping_region,
            );
        }
    }
    fn draw(&mut self, widget: &Rc<dyn WidgetI>, clipping_region: Option<Layout>) {
        if let Some(widget) = widget.as_any().downcast_ref::<Container>() {
            self.draw_widget(widget, NO_CUSTOM, clipping_region);

            // Set clipping region for scrollbars (if any)
            let clipping_region = if let Some(scroll) = widget.scrollbar.as_ref() {
                let (x, y) = scroll;
                let widget_base = widget.base();

                // When scrollbars are placed they take up space
                // and we want to leave room for them
                let x_buffer = if x.base().layout.w > 0.0 {
                    x.base().layout.h
                } else {
                    0.0
                } + x.buffer;
                let buffered_h = ((widget_base.layout.y + widget_base.layout.h) - x_buffer).abs();
                let y_buffer = if y.base().layout.h > 0.0 {
                    y.base().layout.w
                } else {
                    0.0
                } + y.buffer;
                let buffered_w = ((widget_base.layout.x + widget_base.layout.w) - y_buffer).abs();

                Some(Layout {
                    x: widget_base.layout.x,
                    y: widget_base.layout.y,
                    w: buffered_w,
                    h: buffered_h,
                })
            } else {
                None
            };

            // Children must always sit atop their parents
            for child in &widget.children {
                self.draw(child, clipping_region);
            }

            // Scrollbar must sit atop everything
            if let Some(scrollbar) = &widget.scrollbar {
                self.draw_widget(&scrollbar.0, NO_CUSTOM, None);
                self.draw_widget(&scrollbar.1, NO_CUSTOM, None);
            }
        } else if let Some(widget) = widget.as_any().downcast_ref::<Canvas>() {
            self.draw_canvas(widget, clipping_region);
        } else {
            self.draw_widget(widget.as_ref(), NO_CUSTOM, clipping_region);
        }
    }
}
impl Renderer for PixelsRenderer {
    fn dirty_clear(&mut self, x: f64, y: f64, h: f64, w: f64) {
        let frame_width = self.pixels.texture().width();
        let frame = self.pixels.frame_mut();

        let color: [u8; 4] = TRANSPARENT.into();
        for y in y as i32..(y + h).round() as i32 {
            for x in x as i32..(x + w).round() as i32 {
                // Ignore drawing pixels off screen
                if x < 0 || y < 0 {
                    continue;
                }

                // Row major layout follows this formula
                let idx = row_major(x as u32, y as u32, frame_width);
                if idx + 3 < frame.len() {
                    frame[idx..idx + 4].copy_from_slice(&color);
                }
            }
        }
    }
    fn clear(&mut self) {
        let color: [u8; 4] = TRANSPARENT.into();
        let frame = self.pixels.frame_mut();
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&color);
        }
    }
    fn present(&mut self) {
        self.pixels.render().unwrap();
    }
    fn draw(&mut self, widget: &Rc<dyn WidgetI>) {
        self.draw(widget, None);
    }
}
