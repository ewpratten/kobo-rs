use kobo::input::touch::TouchEventListener;

fn main() {
    // Open a touch event listener
    println!("Opening touch event listener...");
    let listener = TouchEventListener::open().unwrap();

    // Read and dump events
    println!("Reading events");
    loop {
        let touch = listener.next_touch(None);
        println!("{:?}", touch);
    }
}
