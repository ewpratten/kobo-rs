
/// Text alignment definitions
#[derive(Debug, Clone)]
pub enum TextAlign {
    /// Align text to the left
    Left,
    /// Align text to the center
    Center,
    /// Align text to the right
    Right,
}

impl Default for TextAlign {
    fn default() -> Self {
        TextAlign::Left
    }
}
