use std::sync::mpsc::{SyncSender, Receiver};
use std::sync::mpsc;
use std::thread;

fn main() {
  // we can make a synchronous channel with 'sync_channel' function.
  // it takes an argument which is a 'usize'. it specifies size of the buffer. 
  // a synchronous channel has a fixed size buffer.  
  // 'sync_channel' returns a tuple with 2 memebers. the first memeber is a 'SyncSender<T>' 
  // and the second one is a 'Receiver<T>'.
  // 'SyncSender<T>' is 'Clone' but 'Receiver<T>' is not.
  // if the buffer is full a send will be blocked until the buffer has free space.
  // passing zero to this method will make this channel a 'rendezvous channel'. each send will be blocking.
  let (tx, rx): (SyncSender<String>, Receiver<String>) = mpsc::sync_channel(0); // passing zero to this method will make this channel a 'rendezvous channel'. each send will be blocking.
  //let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel::<String>();

  let tx_clone = tx.clone(); // because 'spawn' receives a 'move closure', and because we want have access to 'tx' after spawning this thread (for spawing more threads), we make a 'clone' for the thread.
  thread::spawn(move || {
    tx_clone.send("Hello from channel.".to_string()).unwrap(); // we will be blocked here for 2 seconds.
    println!("sent first message."); // this message won't be printed for 2 seconds.
    tx_clone.send("Goodbye from channel.".to_string()).unwrap();

    println!("the thread is finished!");
  });

  println!("main thread sleeping for 2 seconds.");
  thread::sleep_ms(2000);
  println!("{}", rx.recv().ok().unwrap());
  println!("received first message.");
  println!("{}", rx.recv().ok().unwrap());
}
