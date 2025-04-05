//! Implement a TokenBucket class that has two API:
//!     1. consume 
//!     2. try_consume
//! This class can be configured with initial capacity, max capacity and refill interval.
//! The difference between the two API is that try_consume is nonblocking when consume is blocking
//! 
//! TODO Add testcases. How can you detect deadlocks, or unexpected latencies in tests?

use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

struct TokenBucket {
    num_tokens: Mutex<usize>,
    cvar: Condvar,
    max_capacity: usize,
    refill_interval: u64, // in msecs
    //refill_thread_handle: Option<Handle>,
}
impl TokenBucket {
    pub fn new(initial_capacity: usize, max_capacity: usize, refill_interval: u64) -> Arc<Self> {
        let token_bucket = Arc::new(Self {
            num_tokens: Mutex::new(initial_capacity),
            cvar: Condvar::new(),
            max_capacity,
            refill_interval,
            //refill_thread_handle = None
        });

        let token_bucket_clone = Arc::clone(&token_bucket);

        // This is the refill thread that is responsible for adding a token
        // for every refill interval.
        // TODO Need to store the handle and join. This is not necessary as the thread will
        // be cleaned up when the main thread exits anyway.
        let _handle = thread::spawn(move || {
            loop {
                {
                    let num_tokens_lock = &token_bucket_clone.num_tokens;
                    let mut num_tokens = num_tokens_lock.lock().unwrap();
                    if *num_tokens < token_bucket_clone.max_capacity {
                        *num_tokens += 1;
                        token_bucket_clone.cvar.notify_one();
                    }
                }
                thread::sleep(Duration::from_millis(token_bucket_clone.refill_interval));
            }
        });

        token_bucket
    }

    // Blocking API
    pub fn consume(&self) {
        // decrement num_tokens if > 0
        let mut num_tokens = self.num_tokens.lock().unwrap();
        while *num_tokens == 0 {
            num_tokens = self.cvar.wait(num_tokens).unwrap();
        }
        *num_tokens -= 1;
        println!("Got token!");
    }

    // Nonblocking API
    pub fn try_consume(&self) -> Option<()> {
        let mut num_tokens = self.num_tokens.lock().unwrap();
        if *num_tokens > 0 {
            *num_tokens -= 1;
            println!("Try: Got token!");
            Some(())
        }
        else {
            println!("Try: Unable to get token!");
            None
        }
    }
}


fn main() {
    let token_bucket = TokenBucket::new(5, 10, 2000);
    let num_threads = 20;
    let mut handles = Vec::new();
    // Create threads that call the consume() API
    for _ in 0..num_threads {
        let token_bucket_clone = Arc::clone(&token_bucket);
        handles.push( thread::spawn(move || {
            let id = thread::current().id();
            println!("{:?}: Enter", id);
            token_bucket_clone.consume();
            println!("{:?}: Exit", id);
        })
    );
}
// Create threads that call the try_consume() API
for _ in 0..num_threads {
    let token_bucket_clone = Arc::clone(&token_bucket);
    handles.push( thread::spawn(move || {
        let id = thread::current().id();
        println!("Try {:?}: Enter", id);
        token_bucket_clone.try_consume();
        println!("Try {:?}: Exit", id);
    })
);
}
for handle in handles {
    handle.join().unwrap();
}
//token_bucket.refill_thread_handle.join();
}