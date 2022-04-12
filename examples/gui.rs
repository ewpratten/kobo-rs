use kobo::{
    coords::{pix_to_chr, CharSpaceCoord},
    display::{
        chr_display::CharacterDisplay,
        chr_gui::{button::Button, text_align::TextAlign, title_bar::TitleBar},
    },
    input::touch::TouchEventListener,
};

fn main() {
    // Open access to the display
    println!("Opening display...");
    let mut display = CharacterDisplay::open().unwrap();
    let touchscreen = TouchEventListener::open().unwrap();

    // Clear the display
    display.clear_screen().unwrap();

    // Components
    let mut title_bar = TitleBar::new("kobo-rs GUI demo", TextAlign::Center);
    let title_cycle_button = Button::new("Cycle title text", CharSpaceCoord::new(3, 7), None);

    // Loop and let the user cycle through demos
    loop {
        // Render everything
        title_bar.render(&mut display).unwrap();
        title_cycle_button.render(&mut display).unwrap();

        // Wait for a screen tap and check the button actions
        loop {
            let touch = touchscreen
                .next_touch(display.get_screen_size_px(), None)
                .unwrap();

            // Handle the title style change button
            if title_cycle_button.is_within(&touch.position, &display) {
                // Cycle the title bar alignment
                title_bar.set_text_align(match title_bar.get_text_align() {
                    TextAlign::Left => TextAlign::Center,
                    TextAlign::Center => TextAlign::Right,
                    TextAlign::Right => TextAlign::Left,
                });
                break;
            }
        }
    }
}
