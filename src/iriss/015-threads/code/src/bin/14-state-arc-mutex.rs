use std::thread::spawn;
use std::sync::{Arc, Mutex};

fn main() {
    let mut data = Arc::new(Mutex::new(Vec::with_capacity(10)));

    let handles = (0..10).map(|_| {
        // We'll clone the arc and move it into the thread
        let cloned_arc = data.clone();
        spawn(move || {
            // Arc also impls Deref for its containing type so we can call lock
            // on the Mutex from the Arc
            let mut guard = cloned_arc
                .lock()
                .expect("another thread with the lock panicked");
            guard.push("Thread reporting in!".to_string());
        })
    });

    handles.for_each(|handle| handle.join().expect("thread panicked"));

    let guard = data.lock().unwrap();
    assert_eq!(guard.len(), 10);
    guard
        .iter()
        .for_each(|s| assert_eq!(s, &"Thread reporting in!".to_string()));
}
