use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    let handler = spawn(|| {
        for i in 0..10 {
            println!("Child iteration: {i}");
            sleep(Duration::from_millis(1));
        }
    });

    for i in 0..10 {
        println!("Main iteration: {i}");
        sleep(Duration::from_millis(1));
    }

    handler.join().expect("Child thread panicked");
}
