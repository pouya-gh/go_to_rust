// an 'Arc' is a thread-safe reference counter.
// by thread-safe i mean its safe to share it between
// threads but it doesn't mean its safe to mutate its value. see 'Mutex'.
// it is used to share a state between many threads.
// each thread has its own copy. 'Arc' knows how many
// copies of its value is out there and won't drop it 
// untill every single copy is droped.

// a 'Mutex' is a lock that holds a value, and when
// a thread has access to its value, other threads can not
// access it. 
// it is used to safely mutate a value in multiple threads.

// 'Mutex' itself is not shareable. we make it shareable 
// by combining it with an 'Arc'.
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
  let a_state = Arc::new(Mutex::new(0));
  let an_array = Arc::new([11, 22, 33, 55, 88]);

// first example. 

// make a clone of our array so we can pass it to 
// the thread and still have access to it.
  let arr = an_array.clone();
  let contains_arr = thread::spawn(move || {
    for elem in &arr[..] {
      print!("{},", elem);
    }
    println!("");
  });

// we still have access to our array.
  println!("{}", an_array[2]);
  contains_arr.join().unwrap();

// second example.

// make two threads. one of them increases the mutex value by 1 (producer).
// the other one decreases the mutex value by 1 (consumer).

  let for_consumer = a_state.clone();
  let consumer = thread::spawn(move || {
    for _ in 0..20000 {
      let mut val = for_consumer.lock().unwrap(); // 'val' must be mutable (why?).
      *val -= 1; // we have to dereference 'val' so we can access the actual value.
    }
  });

  let for_producer = a_state.clone();
  let producer = thread::spawn(move || {
    for _ in 0..20000 {
      let mut val = for_producer.lock().unwrap();
      *val += 1;
    }
  });

  consumer.join().unwrap();
  producer.join().unwrap();
  println!("{}", *a_state.lock().unwrap()); // it should print 0 here.
}
