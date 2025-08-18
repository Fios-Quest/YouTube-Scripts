static mut HELLO_MESSAGE: &str = "Hello!";

fn main() {
    another_function();

    // SAFETY: We only ever modify this variable from the main thread,
    // HELLO_MESSAGE is never used by other threads
    unsafe {
        HELLO_MESSAGE = "CHANGED!";
    }

    another_function();
}

fn another_function() {
    // SAFETY: This function is only called in the main thread
    unsafe {
        println!("HELLO_MESSAGE is currently: {HELLO_MESSAGE}");
    }
}
