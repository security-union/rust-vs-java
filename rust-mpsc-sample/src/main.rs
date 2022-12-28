use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    // Start a new thread that will send a string
    thread::spawn(move || {
        let msg = "hello from a new thread!".to_string();
        tx.send(msg).unwrap();
    });

    // Wait for the message in the main thread
    let received = rx.recv().unwrap();
    println!("Received: {}", received);
}
