use std::{sync::{Arc, Mutex}, thread};

fn main() {
    let lock1 = Arc::new(Mutex::new(1));
    let lock2 = Arc::new(Mutex::new(2));
 
    let lock1_clone = lock1.clone();
    let lock2_clone = lock2.clone();
 
    let t1 = thread::spawn(move || {
        let _guard1 = lock1_clone.lock().unwrap();
        println!("Thread 1 acquired lock 1");
        thread::sleep(std::time::Duration::from_secs(1));
        println!("Thread 1 trying to acquire lock 2");
        let _guard2 = lock2_clone.lock().unwrap();
        println!("Thread 1 acquired lock 2");
    });
 
    let t2 = thread::spawn(move || {
        let _guard2 = lock2.lock().unwrap();
        println!("Thread 2 acquired lock 2");
        thread::sleep(std::time::Duration::from_secs(1));
        println!("Thread 2 trying to acquire lock 1");
        let _guard1 = lock1.lock().unwrap();
        println!("Thread 2 acquired lock 1");
    });
 
    t1.join().unwrap();
    t2.join().unwrap();
 }