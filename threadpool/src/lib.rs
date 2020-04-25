/// Create a simple library for Thread pool
/// Rust uses OS threads for zero-cost abstraction

use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use std::sync::Arc;

pub struct ThreadPool {
    _handles: Vec<std::thread::JoinHandle<()>>,
    sender: Sender<Box<dyn Fn()+ Send>>,
}

impl ThreadPool {
    pub fn new(num_threads: u8) -> Self {
        // Using Dyn dispatch;
        let (sender, receiver) = channel::<Box<dyn Fn()+ Send>>();
        // Arc allows multiple owners. Mutex allows exclusive access to receiver.
        let receiver = Arc::new(Mutex::new(receiver));
        
        // Why is the closure move?
        let _handles = (0..num_threads).map(|_| { 
            // Increments ref counter for receiver.
            let clone = receiver.clone();
            std::thread::spawn(move || 
            loop {
                // We copy the clone to each thread.
                let work = clone.lock().unwrap().recv().unwrap();
                work();
            })
         })
        .collect();
        Self {
            _handles, sender
        }
    }
    
    pub fn execute<T: Fn() + Send>(&self, work: T) {
        self.sender.send(Box::new(work));
    }
}


#[test]
fn it_works() {
    let pool = ThreadPool::new(10);
    pool.execute(|| println!("Hello from thread!"));
    pool.execute(|| println!("Hello from thread!"));
}


