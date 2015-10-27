// in rust pointers are considered 'unsafe' and they are called 'raw pointers'.
// instead, we could use refrences.

fn does_not_mutate(mut v: i32) { // this functions takes ownership of its arguments. this means you can't use your variables after calling this function, if they don't implement 'Copy' trait (if it's not 'Copy'). 'i32' is copy so we won't have any problems.
//                  ^ this is how we annotate that the argument must be mutable.
  v -= 1;
}

fn this_mutates(v: &mut i32) { // this functions gets a 'mutable reference' and 'borrows' the variable. pay attention to the '&'.
  *v -= 1;
}

fn main() {
  let mut v = 33; 
  println!("the initial value is: {}", v);

  does_not_mutate(v);
  println!("value after calling 'does_not_mutate' function: {}", v);

  this_mutates(&mut v);
  println!("value after calling 'this_mutates' function: {}", v);
}
