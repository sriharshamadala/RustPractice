//! Implementation of the synchronization problem known as rendezvous.
//! This problem is described well in the "Little book of Semaphores"

use std::sync::{Arc, Mutex, Condvar};
use std::{thread, time};

fn main() {

    // a (mutex, condvar) pair to signal when A1 is done
    let a1_sig_pair = Arc::new((Mutex::new(false), Condvar::new()));
    let a1_sig_pair_clone = Arc::clone(&a1_sig_pair);

    // a (mutex, condvar) pair to signal when B1 is done
    let b1_sig_pair = Arc::new((Mutex::new(false), Condvar::new()));
    let b1_sig_pair_clone = Arc::clone(&b1_sig_pair);

    let handle_b = thread::spawn(move || {
        println!("B1");

        // Signal to any waiting thread that B1 is done
        let (b_lock, b_cvar) = &*b1_sig_pair;
        {
            let mut b1_done = b_lock.lock().unwrap();
            *b1_done = true;
        }
        b_cvar.notify_one();

        thread::sleep(time::Duration::from_secs(1));

        // Wait for A1 to finish
        let (lock, cvar) = &*a1_sig_pair;
        let mut a1_done = lock.lock().unwrap();
        while !*a1_done {
            a1_done = cvar.wait(a1_done).unwrap();
        }

        println!("B2");
    });

    let handle_a = thread::spawn(move || {
        println!("A1");

        // Signal to any waiting thread that A1 is done
        let (a_lock, a_cvar) = &*a1_sig_pair_clone;
        {
            let mut a1_done = a_lock.lock().unwrap();
            *a1_done = true;
        }
        a_cvar.notify_one();

        thread::sleep(time::Duration::from_secs(1));

        // Wait for B1 to finish
        let (b_lock, b_cvar) = &*b1_sig_pair_clone;
        let mut b1_done = b_lock.lock().unwrap();
        while !*b1_done {
            b1_done = b_cvar.wait(b1_done).unwrap();
        }

        println!("A2");
    });


    handle_a.join().unwrap();
    handle_b.join().unwrap();
}
