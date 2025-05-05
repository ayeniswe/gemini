pub const RED: Color = Color::RGBA(255, 0, 0, 255);
pub const GREEN: Color = Color::RGBA(0, 255, 0, 255);
pub const BLUE: Color = Color::RGBA(0, 0, 255, 255);
pub const WHITE: Color = Color::RGBA(255, 255, 255, 255);
pub const BLACK: Color = Color::RGBA(0, 0, 0, 255);

/// Represents an RGB color using 8-bit red, green, and blue components.
#[derive(Debug, Clone, Copy)]
pub enum Color {
    /// An RGB color in the form (red, green, blue).
    RGBA(u8, u8, u8, u8),
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
impl From<Color> for tiny_skia::Color {
    fn from(color: Color) -> Self {
        match color {
            Color::RGBA(r, g, b, a) => tiny_skia::Color::from_rgba8(r, g, b, a),
        }
    }
}
