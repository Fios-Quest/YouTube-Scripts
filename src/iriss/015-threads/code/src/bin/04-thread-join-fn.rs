use std::thread::spawn;

fn child() {
    println!("This is also a child thread");
}

fn main() {
    println!("This is the main thread");
    let handler = spawn(child);
    handler.join().expect("Child thread panicked");
    println!("This is the end of the main thread");
}
