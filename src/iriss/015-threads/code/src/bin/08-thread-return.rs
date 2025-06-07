use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    let data = u16::MIN..u16::MAX;
    let handler = spawn(move || data.map(|i| i as u32).sum::<u32>());

    while !handler.is_finished() {
        println!("Still working!");
        sleep(Duration::from_nanos(1));
    }

    let answer = handler.join().expect("Child thread panicked");
    assert_eq!(answer, 2147385345);
}
