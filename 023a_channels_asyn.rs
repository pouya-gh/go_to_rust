// a channel is an one-way line of communication bewteen
// threads.
// channels are like pipes in 'inter-process communication' 
// in C.

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

fn main() {
  // we can make an asynchronous channel with 'channel' function.
  // it returns a tuple with 2 memebers. the first memeber is a 'Sender<T>' 
  // and the second one is a 'Receiver<T>'.
  // 'Sender<T>' is 'Clone' but 'Receiver<T>' is not.
  let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
  // let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel::<String>();

  let tx_clone = tx.clone(); // because 'spawn' receives a 'move closure', and because we want have access to 'tx' after spawning this thread (for spawing more threads), we make a clone of it.
  thread::spawn(move || {
    tx_clone.send("Hello from channel.".to_string()).unwrap();
    println!("sent first message.");
    tx_clone.send("How are you?".to_string()).unwrap();
    println!("sent second message.");
    println!("thread is sleeping for 2 seconds.");
    thread::sleep_ms(2000); // sleep for 2 seconds.
    tx_clone.send("Goodbye from channel.".to_string()).unwrap();

    println!("the thread is finished!");
  });

  println!("{}", rx.recv().ok().unwrap());
  println!("received first message.");
  println!("{}", rx.recv().ok().unwrap());
  println!("received second message.");
  println!("{}", rx.recv().ok().unwrap()); // this will be blocked for 2 seconds.

  // note that the second read from channel will be blocked 
  // only if channels 'buffer' is empty.
  // if the thread had sent 3 messages before going to
  // sleep, this program would end immediately. 
}
