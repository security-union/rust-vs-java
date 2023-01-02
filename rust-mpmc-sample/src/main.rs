use std::thread;
use crossbeam::channel::{unbounded};

fn main() {
    // Create an unbounded MPMC channel.
    let (sender, receiver) = unbounded();

    // Spawn producer threads.
    for i in 0..5 {
        let sender = sender.clone();
        thread::spawn(move || {
            let _ = sender.send(i * 2);
        });
    }

    // Spawn consumer threads.
    let mut handles = Vec::new();
    for _ in 0..5 {
        let receiver = receiver.clone();
        handles.push(thread::spawn(move || {
            let val = receiver.recv().unwrap();
            println!("received value: {}", val);
        }));
    }
    // Wait for all threads to finish.
    for handle in handles {
        if let Err(e) = handle.join() {
            println!("thread panicked: {:?}", e);
        }
    }
}
