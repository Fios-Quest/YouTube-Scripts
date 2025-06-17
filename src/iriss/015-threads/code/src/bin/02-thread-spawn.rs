use std::thread::spawn;

fn main() {
    println!("This is the main thread");
    spawn(|| {
        println!("This is a child thread");
    });
    println!("This is the end of the main thread");
}
