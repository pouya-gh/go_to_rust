fn fib(n: i64) -> i64 {
  if n == 1 || n == 0 {
    return 1;
  } else if n < 0 {
    return -1;
  } else {
    return fib(n-1) + fib(n-2); // sometimes the borrow checker does not allow this. read about borrow checker to find out when.
  }
}

fn main() {
  println!("{}", fib(5));
}
