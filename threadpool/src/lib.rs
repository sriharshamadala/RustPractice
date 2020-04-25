/// Create a simple library for Thread pool
/// Rust uses OS threads for zero-cost abstraction

use std::sync::mpsc::channel;
use std::sync::Mutex;

pub struct ThreadPool {
    _handles: Vec<std::thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(num_threads: u8) -> Self {
        // Using Dyn dispatch;
        let (sender, receiver) = channel::<Box<dyn Fn()>>();
        let receiver = Mutex::new(receiver);
        
        // Why is the closure move?
        let _handles = (0..num_threads).map(|_| { std::thread::spawn(move || 
            loop {
                // Blocking call. recv() returns Result<>. Ideally we want to handle Err<>;
                // Mutex also shouldn't be unwrapped; If any mutex holder crashes, everthing crashes.
                let work = receiver.lock().unwrap().recv().unwrap();
                work();
            })
         })
        .collect();
        Self {
            _handles
        }
    }
    
    pub fn execute<T: Fn()>(&self, work: T) {
    }
}


#[test]
fn it_works() {
    let pool = ThreadPool::new(10);
    pool.execute(|| println!("Hello from thread!"));
    pool.execute(|| println!("Hello from thread!"));
}


