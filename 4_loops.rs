fn main() {

  println!("executing for loop:");
  for i in 0..10 { // expression after 'in' must be an 'iterator'
    if i == 5 {
      continue;
    } else if i == 10 {
      break;
    }
    println!("{}", i);
  } 

  println!("executing while loop:");
  let mut done: bool = false;
  while !done {
    println!("while loop done!");
    done = true;
  }

  // and this is just an infinite loop
  // loop { println!("runs for ever!");  }  
}
