fn get_a_closure_stk<F>(the_closure: F) // taking a closure with static dispatch
  where F : Fn() {
  the_closure()
}

fn get_a_closure_dyn(the_closure: &Fn()) { // taking a closure with dynamic dispatch
//                                ^
  the_closure()
}

fn return_a_closure() -> Box<Fn(i32) -> i32> {
  let num: i32 = 2;

  Box::new(move |x: i32| -> i32 { // with the key word 'move' variable 'num' is 'moved' to the stack frame of the closure.
    x * num
  }) // no need for ';'. because we want to return this value.
}

fn main() {
  // no need to annotate the types of arguments and return values. 
  let a_closure = |x| x + 1;
  // let a_closure = |x: i32| -> i32 { x + 1 };
  let another_closure = |x: i32| -> i64 {
    x as i64 // no ';' because we want to return a value here.
  }; // DO NOT FORGET ';'. 'let' always expects a ';'.
  let a_returned_closure = return_a_closure();

  println!("{}", a_closure(2));
  println!("{}", another_closure(-23i32)); 

  get_a_closure_stk(|| println!("Hello! (stk)"));
  // get_a_closure_stk(move || println!("Hello! (stk)")); // yes! we could send a 'move closure' too.
  get_a_closure_dyn(&|| println!("Hello! (dyn)"));
  //                ^

  println!("{}", a_returned_closure(3));
  println!("{}", a_returned_closure(2));
}