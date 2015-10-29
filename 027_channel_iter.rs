// we can iterate over values in a channel's 
// buffer. method 'iter' returns an iterator
// we can use in a for loop.
// if the buffer is empty the loop will hold.
// if the buffer is closed the loop will end.

use std::sync::mpsc;
use std::thread;

fn main() {
  let (tx, rx) = mpsc::channel::<i32>();

  let tx_clone = tx.clone();
  thread::spawn(move || {
    thread::sleep_ms(1000);
    tx_clone.send(1).unwrap();
    thread::sleep_ms(1000);
    tx_clone.send(22).unwrap();
    thread::sleep_ms(1000);
    tx_clone.send(333).unwrap();
    drop(tx_clone);
  });

  // if we want the iteration to end we must close 
  // the channel. we can close a channel simply by
  // droping its 'sender' and 'receiver'.
  drop(tx); // must drop tx here too unless the loop will wait for ever.

  for ii in rx.iter() {
    println!("{}", ii);
  }
}
