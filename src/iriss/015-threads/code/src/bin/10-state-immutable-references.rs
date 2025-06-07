use std::thread::scope;

fn main() {
    let mut data = String::from("This data is owned by the main thread");

    scope(|s| {
        (0..5).for_each(|_| {
            s.spawn(|| println!("Thread accessing data {}", &data));
        });
    });

    data.push_str(" still");

    assert_eq!(&data, "This data is owned by the main thread still");
}
