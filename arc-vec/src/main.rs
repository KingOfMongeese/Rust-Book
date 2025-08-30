#![warn(clippy::pedantic)]

use std::sync::{Arc, Mutex};
use std::thread;
use rand::prelude::*;

fn main() {

    // define our shared data
    let shared_state: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(Vec::new()));

    // define container to hold our handles
    let mut handles = vec![];

    // 5 threads
    for _ in 0..5 {

        // clone our shared state, ie make another pointer to our shared state, ARC counts this pointer
        // we are not cloning the mutex and data inside
        let shared_state = Arc::clone(&shared_state);

        // spawn a new thread
        let handle = thread::spawn(move || {

            // new rng
            let mut rng = rand::rng();

            // add 10 numbers
            for _ in 0..10 {

                // the range to add the numbers from
                // defined here to avoid duplicating definitions
                let range_to_add = 0..100;
                let mut num_to_add = rng.random_range(range_to_add.clone());

                // aqcuire a lock on our data, this blcoks all other threads trying to lock
                // explicitly waitied until we absolutely need the data to lock
                let mut data = shared_state.lock().unwrap();

                // check for duplicates
                while data.contains(&num_to_add) {
                    num_to_add = rng.random_range(range_to_add.clone());
                }
                // push the data
                data.push(num_to_add);
            } // mutex unlocked at the end of each loop iter, since its dropped

        });

        // append thread, even tho thread handle defined earlier, thread may still be running when main gets here
        handles.push(handle);
    }

    // wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", shared_state.lock().unwrap());
}
