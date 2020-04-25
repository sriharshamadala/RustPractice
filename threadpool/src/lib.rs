/// Create a simple library for Thread pool
/// Rust uses OS threads for zero-cost abstraction

use std::sync::mpsc::channel;

pub struct ThreadPool {
    _handles: Vec<std::thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(num_threads: u8) -> Self {
        // Channel expects the type we will send over. 
        // We want to pass a function using this channel so that the threads can execute it.
        // But Fn() is a trait and hence not of fixed size; hence cannot be used Channel<Fn()>
        // We use the notion of dyn dispatch where Fn() is instead defined in heap; since we are only referring it is fixed size.
        let (sender, receiver) = channel::<Box<dyn Fn()>>();
        /* Convert this logic to map
        for _ in 0..num_threads {
            let handle = std::thread::spawn(|| {});
        }
        */
        let _handles = (0..num_threads).map(|_| { std::thread::spawn(|| 
            loop {
                // Blocking call. recv() returns Result<>. Ideally we want to handle Err<>;
                // Correctly we get an error that receiver is not thread-safe. 
                // It is written for single consumer, which isn't the case here.
                let work = receiver.recv().unwrap();
                work();
            })
         })
        .collect();
        Self {
            _handles
        }
    }
    
    // Instead of &self, if we use self here, thread_pool.execute()
    // can only be called once since this function takes ownership
    // of ThreadPool and won't return until complete.
    pub fn execute<T: Fn()>(&self, work: T) {
    }
}


#[test]
fn it_works() {
    let pool = ThreadPool::new(10);
    pool.execute(|| println!("Hello from thread!"));
    pool.execute(|| println!("Hello from thread!"));
}


