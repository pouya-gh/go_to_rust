// UNSTABLE! compile me with nightly rust.

// we could use 'select!' macro to wait on 
// multiple threads and execute something
// based on wich on which one them is 
// returned first.


#![feature(mpsc_select)]

use std::thread;
use std::sync::mpsc;

// two placeholder functions for now
fn long_running_thread() { thread::sleep_ms(1000); }
fn calculate_the_answer() -> u32 { 42 }

let (tx1, rx1) = mpsc::channel(); // rust can figure out the type of data we want to send and receive.
let (tx2, rx2) = mpsc::channel();

thread::spawn(move|| { long_running_thread(); tx1.send(()).unwrap(); });
thread::spawn(move|| { tx2.send(calculate_the_answer()).unwrap(); });

select! {
    _ = rx1.recv() => println!("the long running thread finished first"),
    answer = rx2.recv() => {
      println!("the answer was: {}", answer.unwrap());
    }
}
