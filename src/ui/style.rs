use super::color::ColorState;

/// A struct representing the visual style of a UI element.
///
/// The `Style` struct defines the appearance and styling properties of a UI
/// element, including its color, corner radius, and optional grid layout.
/// This struct is typically used  to customize the visual representation of
/// widgets or containers, allowing for consistent styling across different
/// elements.
///
/// - `color`: Defines the color of the UI element, typically used for the
///   background, text, or other visual components.
/// - `radius`: Specifies the corner radius (rounded corners) for the UI
///   element. This value controls how rounded the corners of the element
///   should be.
/// - `grid`: Optionally defines a `Grid` layout for the element. If present,
///   this field indicates that the element follows a grid-based structure
///   (e.g., for a container widget with a grid of items or cells).
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Style {
    pub color: ColorState,
    pub radius: u32,
}
