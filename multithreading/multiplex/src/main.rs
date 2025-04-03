//! This is a solution to the multiplex problem defined in the "little book of semaphores"
//! Imagine a nightclub that has a capacity of 100 people and there 1000 people
//! waiting to enter. Each person can stay for an arbitrary amount of time
//! We have to make sure that at any given time not more than 100 people are in the club
//! 

use std::sync::{Arc, Mutex, Condvar};
use std::{thread, time};
use rand::prelude::*;

// Nightclub capacity
const CAPACITY: usize = 10;
// Number of people waiting to get in
const QUEUE_LEN: usize = 100;

fn main() {
    let mut handles = Vec::new();
    let club_full_sig_pair = Arc::new((Mutex::new(0usize), Condvar::new()));

    for _ in 0..QUEUE_LEN {
        let club_full_sig_pair_clone = Arc::clone(&club_full_sig_pair);
        handles.push(thread::spawn(move || {

            let id = thread::current().id();
            let mut rng = rand::rng();

            // Check the current occupancy of the club
            let (lock, cvar) = &*club_full_sig_pair_clone;
            let mut club_occupancy = lock.lock().unwrap();

            // Wait if the club is full
            while *club_occupancy >= CAPACITY {
                club_occupancy = cvar.wait(club_occupancy).unwrap();
            }

            *club_occupancy += 1;

            println!("{:?} Entered!", id);
            println!("Occupancy = {}", *club_occupancy);
            // Release the mutex for the other threads to make progress
            drop(club_occupancy);

            thread::sleep(time::Duration::from_secs(rng.random_range(1..3)));

            // Decrement the occupancy
            {
                let mut club_occupancy = lock.lock().unwrap();
                *club_occupancy -= 1;
                println!("{:?} Left!", id);
                println!("Occupancy = {}", *club_occupancy);
            }

            // Notify a waiting thread that they can enter
            cvar.notify_one();
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}