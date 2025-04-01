//! Here we implement a scenario where threads are asynchronously incrementing a shared counter

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..10 {
        // The reason we Arc::clone() as opposed to counter.clone() is that
        // the compiler doens't know if we want to clone the Arc or the underlying unit
        // as '.' is recursively processed
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        } ));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Value of the counter is {}", *counter.lock().unwrap());
}
