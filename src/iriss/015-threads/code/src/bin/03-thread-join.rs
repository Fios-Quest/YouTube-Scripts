use std::thread::spawn;

fn main() {
    println!("This is the main thread");
    let handler = spawn(|| {
        println!("This is a child thread");
    });
    handler.join().expect("Child thread panicked");
    println!("This is the end of the main thread");
}
