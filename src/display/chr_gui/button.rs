use crate::{
    coords::{pix_to_chr, CharSpaceCoord, PixelSpaceCoord},
    display::{chr_display::CharacterDisplay, color::EinkColor},
};

use super::text_align::TextAlign;

#[derive(Debug)]
pub struct Button {
    text: String,
    size: CharSpaceCoord,
    position: CharSpaceCoord,
}

impl Button {
    /// Construct a new `Button`
    pub fn new(text: &str, position: CharSpaceCoord, button_size: Option<CharSpaceCoord>) -> Self {
        let size = match button_size {
            Some(size) => size,
            None => CharSpaceCoord::new(text.len() as i32 + 2, 3),
        };

        Self {
            text: text.to_string(),
            size,
            position,
        }
    }

    /// Set new text for the `Button`
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    /// Get the text of the `Button`
    pub fn get_text(&self) -> &str {
        &self.text
    }

    /// Render the button to a character display
    pub fn render(&self, display: &mut CharacterDisplay) -> Result<(), std::io::Error> {
        // Render three lines worth of spaces for the button bg
        let mut bg_string = String::new();
        for row in 0..self.size.y {
            for col in 0..(self.size.x - 1) {
                bg_string.push(' ');
            }
            if row == self.size.y - 1 {
                bg_string.push(' ');
            } else {
                bg_string.push('\n');
            }
        }
        display.write_str(
            &bg_string,
            self.position,
            true,
            None,
            Some(EinkColor::GrayA),
        )?;

        // Render the text in the center of the button
        let text_width = self.text.len() as i32;
        let x = (self.size.x / 2) - (text_width / 2) + self.position.x;
        let y = (self.size.y / 2) + self.position.y;
        display.write_str(
            &self.text,
            CharSpaceCoord::new(x, y),
            true,
            None,
            Some(EinkColor::GrayA),
        )?;

        Ok(())
    }

    /// Check if a given coordinate is within the button
    pub fn is_within(&self, coord: &PixelSpaceCoord, display: &CharacterDisplay) -> bool {
        let coord = pix_to_chr(
            coord.clone(),
            display.get_screen_size_px(),
            display.get_screen_size_ch(),
        );
        coord.x >= self.position.x
            && coord.x <= self.position.x + self.size.x
            && coord.y >= self.position.y
            && coord.y <= self.position.y + self.size.y
    }
}
