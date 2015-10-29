use std::thread;

fn say_hi_1300_times(name: String) {
  for i in 0..1300 {
    println!("{}.({}): hi!", i, name);
  }
}

fn main() {
  let a_thread: thread::JoinHandle<()>; // just to show you what is the type of this variable
  let another_thread; // rust can figure out the type.

  a_thread = thread::spawn(move || {
    say_hi_1300_times("A thread".to_string());
  });

  another_thread = thread::spawn(move || {
    say_hi_1300_times("Another thread".to_string());
  });

  a_thread.join().unwrap();
  another_thread.join().unwrap();
}
