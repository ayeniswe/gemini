pub const RED: Color = Color::RGBA(255, 0, 0, 255);
pub const GREEN: Color = Color::RGBA(0, 255, 0, 255);
pub const BLUE: Color = Color::RGBA(0, 0, 255, 255);
pub const WHITE: Color = Color::RGBA(255, 255, 255, 255);
pub const BLACK: Color = Color::RGBA(0, 0, 0, 255);
pub const TRANSPARENT: Color = Color::RGBA(0, 0, 0, 0);

/// Represents an RGB color using 8-bit red, green, and blue components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Color {
    /// An RGB color in the form (red, green, blue).
    RGBA(u8, u8, u8, u8),
}
impl Color {
    /// Source over blend for RGB channels that
    /// have not been premultiplied
    ///
    /// This blends the alpha of a foreground and background
    /// to give a smooth blend effect. The foregound influence
    /// is inversely related by how much of the background is being shown
    /// through the background's opacity
    ///
    /// # Panics
    /// This function will panic if `fg` and `bg` are not exactly
    /// 4 bytes of data
    pub(crate) fn src_over_blend(fg: &[u8], bg: &[u8]) -> [u8; 4] {
        assert!(fg.len() == 4 && bg.len() == 4);

        let bg_r = bg[0] as f32;
        let bg_g = bg[1] as f32;
        let bg_b = bg[2] as f32;
        let bg_a = bg[3] as f32;

        let fg_r = fg[0] as f32;
        let fg_g = fg[1] as f32;
        let fg_b = fg[2] as f32;
        let fg_a = fg[3] as f32 / 255 as f32;

        // Source-over blend
        let out_r = (fg_r * fg_a + bg_r * (1.0 - fg_a))
            .round()
            .clamp(0.0, 255.0) as u8;
        let out_g = (fg_g * fg_a + bg_g * (1.0 - fg_a))
            .round()
            .clamp(0.0, 255.0) as u8;
        let out_b = (fg_b * fg_a + bg_b * (1.0 - fg_a))
            .round()
            .clamp(0.0, 255.0) as u8;

        [out_r, out_g, out_b, 255]
    }
}
impl Default for Color {
    fn default() -> Self {
        WHITE
    }
}
impl From<Color> for (u8, u8, u8, u8) {
    fn from(color: Color) -> Self {
        match color {
            Color::RGBA(r, g, b, a) => (r, g, b, a),
        }
    }
}
// RGB-ONLY ignore Alpha
impl From<Color> for (u8, u8, u8) {
    fn from(color: Color) -> Self {
        match color {
            Color::RGBA(r, g, b, _) => (r, g, b),
        }
    }
}
impl From<Color> for [u8; 4] {
    fn from(color: Color) -> Self {
        match color {
            Color::RGBA(r, g, b, a) => [r, g, b, a],
        }
    }
}
impl From<Color> for Vec<u8> {
    fn from(color: Color) -> Self {
        match color {
            Color::RGBA(r, g, b, a) => vec![r, g, b, a],
        }
    }
}
impl From<[u8; 4]> for Color {
    fn from(color: [u8; 4]) -> Self {
        Color::RGBA(color[0], color[1], color[2], color[3])
    }
}
impl From<Color> for tiny_skia::Color {
    fn from(color: Color) -> Self {
        match color {
            Color::RGBA(r, g, b, a) => tiny_skia::Color::from_rgba8(r, g, b, a),
        }
    }
}
