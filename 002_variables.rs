// define a variable with 'let'
fn main() {
  // variables are immutable by default.
  let a: String = String::new(); // an immutable String 
  let b: i32 = 26; // an immutable 32 bit integer
  // there is no need to specify the type. rust can figure it out. 
  let mut c = 3.14; // a mutable 64 bit float.
  
  println!("{}\n{}\n{}", a, b, c);
}