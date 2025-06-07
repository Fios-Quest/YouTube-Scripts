use std::thread::scope;

fn main() {
    let mut data = String::from("This is owned by the main thread");

    scope(|s| {
        s.spawn(|| {
            data.push_str(" but can be modified by a child thread");
        });
    });

    assert_eq!(
        &data,
        "This is owned by the main thread but can be modified by a child thread"
    );
}
