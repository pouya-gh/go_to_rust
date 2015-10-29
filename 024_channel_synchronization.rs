// we could use channels for synchronizing threads. 

use std::sync::mpsc;
use std::thread;

fn main() {
  let (tx, rx) = mpsc::sync_channel::<bool>(0);

  let done = tx.clone(); 
  thread::spawn(move || {
    println!("thread: doing something very time consuming.");
    thread::sleep_ms(2000);
    println!("the thread is finished!");
    done.send(true).unwrap();
  });

  rx.recv().ok().unwrap();
}
