// in this example we implement a simple pool
// of 3 workers using channels, Arc and Mutex.

use std::sync::mpsc::{Sender, Receiver, RecvError};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

fn worker(id: i32, jobs: Arc<Mutex<Receiver<i32>>>, result: Sender<i32>) {
  let mut job; // rust can figure the type out. ( go to (1) )
  // let mut job: Result<i32, RecvError>;
  loop {
    {
      let temp = jobs.lock().unwrap(); // acquire mutex.
      job = temp.recv(); // (1) here Rust figures out the type of 'job'. 
      // here 'temp' is droped. when that happens the Mutex lock is released.
      // you could explicitly drop 'temp' like this.
      // drop(temp);
    }
    match job {
        Ok(job_data) => {
          println!("worker {} received job {}", id, job_data);
          thread::sleep_ms(1000);
          result.send(job_data * 2).unwrap();
        }
        Err(RecvError) => {
          break;
        }
    }
  }
  drop(result);
}

fn main() {
  let (j_tx, j_rx) = mpsc::channel::<i32>(); // for sending jobs to workers.
  let (r_tx, r_rx) = mpsc::channel::<i32>(); // for receiving results from workers.
  // we need to share j_rx between multiple threads, but Receiver is not 'Clone'.
  // we can use an Arc and a Mutex to solve this problem.
  let j_rx_cloneable = Arc::new(Mutex::new(j_rx));

  // spawn 3 workers.
  for ii in 1..4 {
    let r_tx_clone = r_tx.clone();
    let j_rx_clone = j_rx_cloneable.clone();
    thread::spawn(move || {
      worker(ii, j_rx_clone, r_tx_clone);
    });
  }

  // send 9 jobs to workers.
  for jj in 1..10 {
    j_tx.send(jj).unwrap();
  }
  drop(j_tx); // we don't have any more jobs.

  drop(r_tx);
  for _ in r_rx.iter() { /* no-op */ }
}
