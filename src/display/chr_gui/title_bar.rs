use crate::{coords::CharSpaceCoord, display::{chr_display::CharacterDisplay, color::EinkColor}};

use super::text_align::TextAlign;

#[derive(Debug)]
pub struct TitleBar {
    text: String,
    text_align: TextAlign,
}

impl TitleBar {
    /// Construct a new `TitleBar`
    pub fn new(text: &str, align: TextAlign) -> Self {
        Self {
            text: text.to_string(),
            text_align: align,
        }
    }

    /// Set new text for the `TitleBar`
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    /// Get the text of the `TitleBar`
    pub fn get_text(&self) -> &str {
        &self.text
    }

    /// Set the text alignment of the `TitleBar`
    pub fn set_text_align(&mut self, align: TextAlign) {
        self.text_align = align;
    }

    /// Get the text alignment of the `TitleBar`
    pub fn get_text_align(&self) -> TextAlign {
        self.text_align.clone()
    }

    /// Render the titlebar to a character display
    pub fn render(&self, display: &mut CharacterDisplay) -> Result<(), std::io::Error> {
        let mut text = self.text.clone();
        let text_align = self.text_align.clone();

        // Pad the text with spaces to the width of the display
        let display_width = display.get_screen_size_ch().x;
        let text_width = text.len() as i32;

        // Render three lines worth of spaces
        let bg_string = (0..(display_width * 3)).map(|_| ' ').collect::<String>();
        display.write_str(&bg_string, CharSpaceCoord::new(0, 0), true, None, None)?;

        // Align the text
        match text_align {
            TextAlign::Left => {
                display.write_str(&text, CharSpaceCoord::new(1, 1), true, None, None)?;
            }
            TextAlign::Center => {
                let x = (display_width / 2) - (text_width / 2);
                display.write_str(&text, CharSpaceCoord::new(x, 1), true, None, None)?;
            }
            TextAlign::Right => {
                let x = display_width - text_width - 1;
                display.write_str(&text, CharSpaceCoord::new(x, 1), true, None, None)?;
            }
        }

        Ok(())
    }
}
