// define a variable with 'let'
fn main() {
  let a: String = String::new();
  let b: i32 = 26;
  let c = 3.14; // there is no need to specify the type. rust can figure it out. (here: f64)
  println!("{}\n{}\n{}", a, b, c);
}