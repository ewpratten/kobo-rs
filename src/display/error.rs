use nalgebra::Vector2;

#[derive(Debug, thiserror::Error)]
pub enum DisplayError {
    #[error("Could not find FBInk executable")]
    FBInkNotFound,
    #[error("Out of bounds: {0}")]
    OutOfBounds(Vector2<i32>),
}
