use std::thread::spawn;

fn main() {
    let handler = spawn(|| {
        for i in 0..5 {
            println!("Child iteration: {i}");
        }
    });

    for i in 0..5 {
        println!("Main iteration: {i}");
    }

    handler.join().expect("Child thread panicked");
}
