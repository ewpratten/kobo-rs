use nalgebra::Vector2;

/// Pixel-space coordinates.
pub type PixelSpaceCoord = Vector2<i32>;

/// Character-space coordinates.
pub type CharSpaceCoord = Vector2<i32>;

/// Convert a pixel-space coordinate to a character-space coordinate.
pub fn pix_to_chr(
    pixel_coord: PixelSpaceCoord,
    screen_size_pixels: PixelSpaceCoord,
    screen_size_chars: CharSpaceCoord,
) -> CharSpaceCoord {
    let pixel_coord_x = pixel_coord.x;
    let pixel_coord_y = pixel_coord.y;
    let screen_size_pixels_x = screen_size_pixels.x;
    let screen_size_pixels_y = screen_size_pixels.y;
    let screen_size_chars_x = screen_size_chars.x;
    let screen_size_chars_y = screen_size_chars.y;

    let char_coord_x =
        (pixel_coord_x as f32 / screen_size_pixels_x as f32) * screen_size_chars_x as f32;
    let char_coord_y =
        (pixel_coord_y as f32 / screen_size_pixels_y as f32) * screen_size_chars_y as f32;

    CharSpaceCoord::new(char_coord_x as i32, char_coord_y as i32)
}
