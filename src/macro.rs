/// Implements the [`Widget`] trait for a struct with common UI fields.
///
/// This macro generates an implementation of the `Widget` trait for a given type,
/// assuming it has the following fields:
///
/// - `pos: (u32, u32)` – The (x, y) position of the widget
/// - `width: u32` – The width of the widget
/// - `height: u32` – The height of the widget
/// - `radius: u32` – The corner radius
/// - `label: Option<String>` – An optional label text
/// - `color: Color` – The fill color
///
/// # Example
/// ```rust
/// struct Button {
///     pos: (u32, u32),
///     width: u32,
///     height: u32,
///     radius: u32,
///     label: Option<String>,
///     color: Color,
/// }
///
/// impl_widget!(Button);
/// ```
///
/// This macro does **not** implement [`Hoverable`]. Use [`impl_hoverable!`] for that.
#[macro_export]
macro_rules! impl_widget {
    ($type:ty) => {
        impl Widget for $type {
            fn pos(&self) -> (u32, u32) {
                self.pos
            }
            fn pos_mut(&mut self) -> (&mut u32, &mut u32) {
                (&mut self.layout.x, &mut self.layout.y)
            }
            fn height(&self) -> u32 {
                self.layout.h
            }
            fn height_mut(&mut self) -> &mut u32 {
                &mut self.layout.h
            }
            fn width(&self) -> u32 {
                self.layout.w
            }
            fn width_mut(&mut self) -> &mut u32 {
                &mut self.layout.w
            }
            fn label(&self) -> &Option<String> {
                &self.text.label
            }
            fn label_mut(&mut self) -> &mut Option<String> {
                &mut self.text.label
            }
            fn color(&self) -> &Color {
                &self.style.color
            }
            fn color_mut(&mut self) -> &mut Color {
                &mut self.style.color
            }
            fn radius(&self) -> u32 {
                self.style.radius
            }
            fn radius_mut(&mut self) -> &mut u32 {
                &mut self.style.radius
            }
        }
    };
}

/// Implements the [`Hoverable`] trait for a widget type that supports hover state.
///
/// This macro generates an implementation of the `Hoverable` trait for a type
/// that already implements [`Widget`] and has the following fields:
///
/// - `hovered: bool` – Whether the widget is currently hovered
/// - `hover_color: Option<Color>` – The color used when the widget is hovered
///
/// # Example
/// ```rust
/// struct Canvas {
///     // fields required by `impl_widget!`
///     pos: (u32, u32),
///     width: u32,
///     height: u32,
///     radius: u32,
///     label: Option<String>,
///     color: Color,
///     // fields required by `impl_hoverable!`
///     hovered: bool,
///     hover_color: Option<Color>,
/// }
///
/// impl_widget!(Canvas);
/// impl_hoverable!(Canvas);
/// ```
///
/// Use this macro in tandem with [`impl_widget!`] to fully implement
/// both traits on UI components that need hover interaction.
#[macro_export]
macro_rules! impl_hoverable {
    ($type:ty) => {
        impl Hoverable for $type {
            fn hover_color(&self) -> &Option<Color> {
                &self.hover.hover_color
            }
            fn hover_color_mut(&mut self) -> &mut Option<Color> {
                &mut self.hover.hover_color
            }
            fn hovered(&self) -> bool {
                self.hover.hovered
            }
            fn hovered_mut(&mut self) -> &mut bool {
                &mut self.hover.hovered
            }
        }
    };
}
