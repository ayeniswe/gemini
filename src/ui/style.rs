use super::{color::Color, layout::Grid};

#[derive(Default, Debug, Clone)]
pub struct Style {
    pub color: Color,
    pub radius: u32,
    pub grid: Option<Grid>,
}