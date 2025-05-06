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
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Text {
    pub label: String,
}
