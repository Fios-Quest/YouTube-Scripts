// We will create a scope and use that to spawn threads instead of spawning 
// them directly.
use std::thread::scope;

fn main() {
    let mut data = String::from("This data is owned by the main thread");

    // The scope function takes a closure with a single parameter that contains
    // the scope context. You use the context to spawn threads
    scope(|s| {
        (0..10).for_each(|_| {
            // We don't _need_ to track the join handler this time, all scoped
            // threads are joined at the end of the scope closure, but if you
            // want to handle a potential thread panic, you can still do that
            // in a scoped thread, by joining the join_handle you get from
            // the `.spawn` method like you would with an unscoped thread from
            // the `spawn` function.
            s.spawn(|| {
                println!("Thread accessing data {}", &data)
            });
        });
    });

    // All scoped threads are joined before the scope function ends, so we are
    // safe to modify the original data.
    data.push_str(" still");

    assert_eq!(&data, "This data is owned by the main thread still");
}
