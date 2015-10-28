// there are 2 types of errors.

// recoverable erros. also called 'failures'.
// use this type of error handling when it is possible to recover.
// prefer Result over Option.

fn div_using_option(num1: f64, num2: f64) -> Option<f64> {
  if num2 == 0.0 {
    None
  } else {
    Some(num1 / num2)
  }
}

enum DivisionErros {
  DivisionByZero,
}

fn div_using_result(num1: f64, num2: f64) -> Result<f64, DivisionErros> {
  if num2 == 0.0 {
    Err(DivisionErros::DivisionByZero)
  } else {
    Ok(num1 / num2)
  }
}

// the 'try' macro is very helpfull when there are several function calls that return 'Result'.
fn try_mac_example() -> Result<f64, DivisionErros> {
  let res = try!(div_using_result(3.4, 0.0)); // if this fails, the function will return early with the received error.
  Ok(res)
}

// unrecoverable erros. also called 'panics'.
// use this type of error handling when the program should not 
// continue in case of an error.

fn div_using_panic(num1: f64, num2: f64) -> f64 {
  if num2 == 0.0 {
    panic!("you can't divide by zero, friend!");
  }
  num1 / num2
}

fn div_using_assert(num1: f64, num2: f64) -> f64 {
  assert!(!(num2 == 0.0)); // there is also the 'assert_eq' macro. example: assert_eq!(num1, num2); it means to values should be equal.
  num1 / num2
}

fn main() {
  match div_using_option(2.0, 5.0) {
    Some(res) => println!("{}", res),
    None => println!("something went wrong!"),
  }
  // if you want to panic in case of an failure, you can use 'unwrap' method. it returns the value Ok contains if successful.
  div_using_option(3.1, 5.2).unwrap();

  match div_using_result(3.0, 1.0) {
    Ok(res) => println!("{}", res),
    Err(DivisionErros::DivisionByZero) => println!("can't divide by zero!"),
  }
  // you could also do this:
  div_using_result(34.2, 342.2).
                           ok().// converts Result to option.
                           expect("zero is not allowed!"); // this is just like 'unwrap' except it takes a string and prints it in case of a failure.
  match try_mac_example() {
    Ok(res) => println!("{}", res),
    Err(DivisionErros::DivisionByZero) => println!("can't divid by zero!"),
  }

  div_using_panic(2.0, 3.0);
  div_using_assert(2.6, 6.2);
}
