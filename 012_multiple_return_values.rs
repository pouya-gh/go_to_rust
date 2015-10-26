// rust doesn't support multiple return values.
// but we can use tuples to return multiple values.

fn return_a_tuple() -> (i32, String) {
  (432, "Hi!".to_string())
}

fn main() {
  let (num, strng) = return_a_tuple(); // we can 'destructure' the return value.
  // let (num, strng): (i32, String) = return_a_tuple();
  println!("num: {}, strng: {}", num, strng);
}