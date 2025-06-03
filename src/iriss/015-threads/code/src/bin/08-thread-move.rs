use std::thread::spawn;

fn main() {
    let data = vec![1, 2, 3, 4, 5];

    let handler = spawn(move || {
        data
            .into_iter()
            .for_each(|i| println!("Processing item {i} from the main thread"));
    });

    handler.join().expect("Child thread panicked");
}
