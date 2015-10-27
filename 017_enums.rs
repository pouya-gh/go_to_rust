// Rust's Enums are different from C's Enums.
// they are more similar to C Unions.
// but if you like you could use them as C-like enums. 

// the syntax to define variants is the same syntax used to define Structs.
enum AnEnum {
  Nothing,
  Somthing(f32),
  AnotherThing{val: i16},
}

enum CEnum { // starting from zero.
    Zero,
    One,
}

// we could also assign values explicitly.
enum AsciiValues {
  A = 65,
  C = 67,
}

fn main() {
  let pi = AnEnum::Somthing(3.14f32);

  match pi {
    AnEnum::Nothing => println!("Nothing was given."),
    AnEnum::Somthing(num) => println!("{}", num),
    AnEnum::AnotherThing{val: v} => println!("{}", v),
  } // because we covered all possible cases we don't need a default case.

  println!("printing 'CEnum': ");
  println!("Zero: {}, Two: {}", CEnum::Zero as i32, CEnum::One as i32);

  println!("printing 'AsciiValues': ");
  println!("A: {}, C: {}", AsciiValues::A as i32, AsciiValues::C as i32);
}
