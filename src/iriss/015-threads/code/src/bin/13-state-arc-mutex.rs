use std::sync::{Arc, Mutex};
use std::thread::spawn;

fn main() {
    let data = Arc::new(Mutex::new(Vec::with_capacity(5)));
    let handles = (0..5).map(|_| {
        let cloned_arc = data.clone();
        spawn(move || {
            let mut guard = cloned_arc
                .lock()
                .expect("another thread with the lock panicked");
            guard.push("Thread reporting in!".to_string());
        })
    });

    handles.for_each(|handle| handle.join().expect("thread panicked"));

    let guard = data.lock().unwrap();
    assert_eq!(guard.len(), 5);
    guard
        .iter()
        .for_each(|s| assert_eq!(s, &"Thread reporting in!".to_string()));
}
