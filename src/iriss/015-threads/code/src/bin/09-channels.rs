use std::sync::mpsc::channel;
use std::thread::spawn;

fn main() {
    let (sender, receiver) = channel();
    let sending_handlers = (0..5).map(move |id| {
        let cloned_sender = sender.clone();
        spawn(move || {
            cloned_sender
                .send(format!("Reporting in from thread {id}"))
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
