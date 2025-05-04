#[macro_export]
macro_rules! impl_widget {
    ($type:ty) => {
        impl Widget for $type {
            fn pos(&self) -> (u32, u32) {
                self.pos
            }
            fn pos_mut(&mut self) -> &mut (u32, u32) {
                &mut self.pos
            }
            fn height(&self) -> u32 {
                self.height
            }
            fn height_mut(&mut self) -> &mut u32 {
                &mut self.height
            }
            fn width(&self) -> u32 {
                self.width
            }
            fn width_mut(&mut self) -> &mut u32 {
                &mut self.width
            }
            fn label(&self) -> &Option<String> {
                &self.label
            }
            fn label_mut(&mut self) -> &mut Option<String> {
                &mut self.label
            }
            fn hover_color(&self) -> &Option<Color> {
                &self.hover_color
            }
            fn hover_color_mut(&mut self) -> &mut Option<Color> {
                &mut self.hover_color
            }
            fn color(&self) -> &Color {
                &self.color
            }
            fn color_mut(&mut self) -> &mut Color {
                &mut self.color
            }
            fn hovered(&self) -> bool {
                self.hovered
            }
            fn hovered_mut(&mut self) -> &mut bool {
                &mut self.hovered
            }
            fn radius(&self) -> u32 {
                self.radius
            }
            fn radius_mut(&mut self) -> &mut u32 {
                &mut self.radius
            }
        }
    };
}
