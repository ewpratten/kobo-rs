use kobo::{display::sizing::KOBO_AURA_SCREEN_SIZE_PX, input::touch::TouchEventListener};

fn main() {
    // Open a touch event listener
    println!("Opening touch event listener...");
    let listener = TouchEventListener::open().unwrap();

    // Read and dump events
    println!("Reading events");
    loop {
        let touch = listener.next_touch(KOBO_AURA_SCREEN_SIZE_PX, None);
        println!("{:?}", touch);
    }
}
