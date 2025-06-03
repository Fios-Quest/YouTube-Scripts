use std::sync::mpsc::channel;
use std::thread::spawn;

fn main() {
    let (sender, receiver) = channel();

    let thread_ids = 0..10;

    // move sender into the closure
    let sending_handlers = thread_ids.map(move |id| {
        // sender is owned by this closure, we want to pass a copy to each
        // child thread so we'll clone it on each iteration
        let cloned_sender = sender.clone();
        // move the cloned sender to the next thread
        spawn(move || {
            cloned_sender.send(format!("Reporting in from thread {id}"))
                .expect("The Receiver was dropped");
        })
    });

    let receiving_handler = spawn(move || {
        while let Ok(message) = receiver.recv() {
            println!("Received message: {message}");
        }
    });


    sending_handlers.for_each(|h| h.join().expect("A sending thread panicked"));

    receiving_handler.join().expect("receiving thread panicked");
}
