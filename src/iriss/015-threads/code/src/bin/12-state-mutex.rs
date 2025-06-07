use std::sync::Mutex;
use std::thread::scope;

fn main() {
    let data = Mutex::new(Vec::with_capacity(5));
    scope(|s| {
        (0..5).for_each(|_| {
            s.spawn(|| {
                let mut guard = data.lock().expect("another thread with the lock panicked");
                guard.push("Thread reporting in!".to_string());
            });
        });
    });

    let guard = data.lock().unwrap();
    assert_eq!(guard.len(), 5);
    guard
        .iter()
        .for_each(|s| assert_eq!(s, &"Thread reporting in!".to_string()));
}
