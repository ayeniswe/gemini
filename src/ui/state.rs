/// Represents the UI interaction state for a widget.
///
/// This struct stores transient visual state such as
/// whether the widget is currently. It is intended to be
/// used for rendering visual feedback without modifying the
///  underlying widget data.
///
/// - `hovered`: Indicating whether the mouse is currently over the widget.
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct State {
    /// Indicates whether the mouse is currently over the widget
    pub hovered: bool,
}
