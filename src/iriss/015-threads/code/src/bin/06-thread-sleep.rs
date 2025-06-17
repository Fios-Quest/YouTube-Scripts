use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    let handler = spawn(|| {
        for i in 0..5 {
            println!("Child iteration: {i}");
            sleep(Duration::from_millis(100));
        }
    });

    for i in 0..5 {
        println!("Main iteration: {i}");
        sleep(Duration::from_millis(100));
    }

    handler.join().expect("Child thread panicked");
}
