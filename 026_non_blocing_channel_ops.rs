// in synchronous channels we don't always want to wait 
// on sends and receives. sometimes we just want to see
// whether a value is available and do something with it
// and something else if not.

use std::sync::mpsc::{SyncSender, Receiver};
use std::sync::mpsc;
use std::thread;

fn main() {
  let (tx, rx): (SyncSender<String>, Receiver<String>) = mpsc::sync_channel::<String>(1);
  // let (tx, rx) = mpsc::sync_channel::<String>(1);

  let tx_clone = tx.clone();
  thread::spawn(move || {
    tx_clone.try_send("This message will be printed.".to_string()).unwrap(); // try_send is available only for the SyncSender.
    thread::sleep_ms(2000);
    tx_clone.try_send("This message will not be printed.".to_string()).unwrap();
  });

  thread::sleep_ms(1000);
  match rx.try_recv() { // try_recv is available both for Sender and SyncSender.
    Ok(v) => println!("{}", v),
    Err(_) => {/* no-op */}
  }

  match rx.try_recv() {
      Ok(v) => println!("{}", v),
      Err(_) => {/* no-op */}
  }
}
