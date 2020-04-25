/// Create a simple library for Thread pool
/// Rust uses OS threads for zero-cost abstraction

use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use std::sync::Arc;

pub struct ThreadPool {
    _handles: Vec<std::thread::JoinHandle<()>>,
    sender: Sender<Box<dyn FnMut()+ Send>>,
}

impl ThreadPool {
    pub fn new(num_threads: u8) -> Self {
        // Using Dyn dispatch;
        let (sender, receiver) = channel::<Box<dyn FnMut()+ Send>>();
        // Arc allows multiple owners. Mutex allows exclusive access to receiver.
        let receiver = Arc::new(Mutex::new(receiver));
        
        // Why is the closure move?
        let _handles = (0..num_threads).map(|_| { 
            // Increments ref counter for receiver.
            let clone = receiver.clone();
            std::thread::spawn(move || 
            loop {
                // We copy the clone to each thread.
                let mut work = match clone.lock().unwrap().recv() {
                    Ok(work) => work,
                    Err(_) => break,
                };
                work();
            })
         })
        .collect();
        Self {
            _handles, sender
        }
    }
    
    pub fn execute<T: FnMut() + Send + 'static>(&self, work: T) {
        self.sender.send(Box::new(work)).unwrap();
    }
}


#[test]
fn it_works() {
    let pool = ThreadPool::new(10);
    let foo = || {
        std::thread::sleep(std::time::Duration::from_secs(1));
    };
    pool.execute(foo);
    pool.execute(foo);
}

#[test]
fn inc_num_test() {
    let mut n = 42;
    let nref = Arc::new(n);

    let pool = ThreadPool::new(10);

    // Fail to derefMut in arc. It doesn't allow exlusive access like &mut does.
    let foo = || {
        **nref = **nref + 1;
    };
    pool.execute(foo);
    pool.execute(foo);
}

