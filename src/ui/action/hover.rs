use crate::ui::color::Color;

#[derive(Default, Debug, Clone)]
pub struct Hover {
    pub hover_color: Option<Color>,
    pub hovered: bool,
}
