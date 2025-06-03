use std::thread::scope;
use std::sync::Mutex;

fn main() {
    let mut data = Mutex::new(Vec::with_capacity(10));

    scope(|s| {
        (0..10).for_each(|_| {
            s.spawn(|| {
                // .lock() returns a MutexGuard. When it goes out of scope,
                // the lock is dropped. MutexGuard implements Deref and
                // DerefMut for the type inside the Mutex
                let mut guard = data.lock()
                    .expect("another thread with the lock panicked");
                guard.push("Thread reporting in!".to_string());
                // The MutexGuard is dropped after this line
            });
        });
    });

    let guard = data.lock().unwrap();
    assert_eq!(guard.len(), 10);
    guard
        .iter()
        .for_each(|s| assert_eq!(s, &"Thread reporting in!".to_string()));
}
