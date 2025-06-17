use std::thread::spawn;

fn main() {
    let data = vec![1, 2, 3, 4, 5];

    let handler = spawn(move || {
        data.into_iter()
            .for_each(|i| println!("Item {i} is in the child thread"));
    });

    handler.join().expect("Child thread panicked");
}
