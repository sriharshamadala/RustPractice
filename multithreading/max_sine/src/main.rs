//! Given an array of values, compute the sine of each of the values and return the maximum
//! Multiple ways to parallelize this:
//!   1. let threads pick a range of indices, compute the result and updated a shared buffer. 
//!      An aggregator then computes the max value. Even though there is no overlap in 
//!      accessing the shared buffer, we still need a Mutex, because the compiler doesn't know.
//!      At this point, this is not much different from the counter example.
//!   2. use threadpool arch where every compute task is added to a pool and threads 
//!      can asynchronously pickup the task, compute and update a shared buffer
//!   3. Each thread processes one value and returns the sin() value. The main thread 
//!      accumulates the result and computes the max value.
//! 
//! For such a trivial computation, (2) is an overkill. but (1) doesn't scale very well.
//! Going with (3)

use std::thread;
use std::sync::Arc;

fn main() {
    // TODO Generate a large array with rand distributions
    let input:Arc<Vec<f32>> = Arc::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);

    let nf_threads = input.len();

    // Don't need a 
    let mut output = vec![0.0; input.len()];
    let mut handles = Vec::new();

    for index in 0..nf_threads {
        let curr_input = input[index];
        handles.push(thread::spawn( move || {
            curr_input.sin()
        }));
    }

    for handle in handles {
        let result = handle.join().unwrap();
        output.push(result);
    }

    println!("{:?}", output);
    println!("Max value is {}", output.iter().fold(f32::NEG_INFINITY, |acc, &v| acc.max(v)));
}
