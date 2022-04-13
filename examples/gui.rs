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

    // Keep track of the second page
    let mut is_in_info_page = false;

    // Components
    let mut title_bar = TitleBar::new("kobo-rs GUI demo", TextAlign::Center);
    let title_cycle_button = Button::new("Cycle title text", CharSpaceCoord::new(3, 7), None);
    let info_page_button = Button::new("More info", CharSpaceCoord::new(25, 7), None);
    let quit_button = Button::new("Quit", CharSpaceCoord::new(3, 12), None);

    // Loop and let the user cycle through demos
    loop {

        // Render everything
        title_bar.render(&mut display).unwrap();
        if !is_in_info_page {
            title_cycle_button.render(&mut display).unwrap();
            info_page_button.render(&mut display).unwrap();
            quit_button.render(&mut display).unwrap();
        } else {
            // Render info
            display
                .write_str(
                    "This is a demo of the kobo-rs GUI library.",
                    CharSpaceCoord::new(1, 7),
                    false,
                    None,
                    None,
                )
                .unwrap();
            display
                .write_str(
                    "Not a whole lot on its own, but useful for making simple apps on top of Nickel", 
                    CharSpaceCoord::new(1, 9), 
                    false, 
                    None, 
                    None
                )
                .unwrap();
        }

        // Wait for a screen tap and check the button actions
        loop {
            let touch = touchscreen
                .next_touch(display.get_screen_size_px(), None)
                .unwrap();

            if !is_in_info_page {
                // Handle the title style change button
                if title_cycle_button.is_within(&touch.position, &display) {
                    println!("Cycling title text style");
                    // Cycle the title bar alignment
                    title_bar.set_text_align(match title_bar.get_text_align() {
                        TextAlign::Left => TextAlign::Center,
                        TextAlign::Center => TextAlign::Right,
                        TextAlign::Right => TextAlign::Left,
                    });
                    break;
                }

                // Handle the info page button
                if info_page_button.is_within(&touch.position, &display) {
                    println!("Switching to info page");
                    is_in_info_page = true;

                    // Refresh the screen
                    display.clear_screen().unwrap();
                    break;
                }

                // Handle the quit button
                if quit_button.is_within(&touch.position, &display) {
                    println!("Quitting");

                    // Refresh the screen
                    display.clear_screen().unwrap();
                    return;
                }
            } else {
                // In the info page, a tap brings us back
                println!("Switching to main page");
                is_in_info_page = false;

                // Refresh the screen
                display.clear_screen().unwrap();
                break;
            }
        }
    }

}
