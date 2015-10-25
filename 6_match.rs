fn main() {
  let x: i32 = 3;

  match x { // can do pattern matching in cases, there is a return value, also
    1 => println!("one"), // the comma is necessary.
    2 => println!("two"),
    3 => {
      print!("th");
      println!("ree"); // for a return value delete semi-collon.
    } // you have to use brackets for multi-line cases.
    _ => println!("nothing"), // not necessary when you cover all possible cases.
  }
}
