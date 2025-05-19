use ab_glyph::{point, Font as _, FontRef, PxScale, ScaleFont as _};

use super::layout::Point;

pub(crate) const DEFAULT_FONT: &'static [u8; 146004] =
    include_bytes!("../../fonts/Roboto-Regular.ttf");

/// A struct representing the text content of a UI element.
///
/// The `Text` struct is used to store and manage textual information
/// within a UI element. It provides a `label` field that holds an optional
/// string, which can be displayed as part of the UI widget or component.
///
/// - `label`: An optional string that represents the text content to be
///   displayed. If `None`, the element may not display any text, or a
///   default value may be used. If `Some`, the string is the label or text
///   shown on the element.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Text {
    pub label: String,
    pub font_size: f32,
    pub pos: Point,
    pub(crate) valign: bool,
    pub(crate) halign: bool,
}
impl Text {
    /// Get the perfect display height and width for text
    /// based on the font style and kerning included
    pub(crate) fn get_true_dimensions(&self) -> Point {
        let font = FontRef::try_from_slice(DEFAULT_FONT).unwrap();
        let mut caret = point(0.0, self.font_size);
        let scale = PxScale::from(self.font_size);
        for c in self.label.chars() {
            let glyph = font.glyph_id(c).with_scale_and_position(scale, caret);
            caret.x += font.as_scaled(scale).h_advance(glyph.id);
        }

        caret.into()
    }
}
impl Default for Text {
    fn default() -> Self {
        Self {
            label: Default::default(),
            font_size: 12.0,
            pos: Default::default(),
            valign: false,
            halign: false
        }
    }
}
