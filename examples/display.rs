use kobo::{coords::CharSpaceCoord, display::chr_display::CharacterDisplay};

fn main() {
    // Open access to the display
    println!("Opening display...");
    let display = CharacterDisplay::open().unwrap();

    // Clear the display
    display.clear_screen().unwrap();

    // Display a welcome message
    display
        .write_str(
            "                      \n  Hello from kobo-rs! \n                       ",
            CharSpaceCoord::new(12, 30),
            true,
            None,
            None,
        )
        .unwrap();
}
