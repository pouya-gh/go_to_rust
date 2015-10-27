// there are 3 types of structs.

#[derive(Debug)]
struct EmptyStruct; // 'unit-like' struct or 'unit'

#[derive(Debug)]
struct TupleStruct(i32, String); // tuple struct

#[derive(Debug)]
struct OldStruct {
  num: i32, // 'mut' keyword doesn't work here.
  flt: f64,
}

fn main() {
  let ts: TupleStruct = TupleStruct(12, "Hi".to_string());
  let os: OldStruct = OldStruct { num: 324, flt: 43.4 };
  let es: EmptyStruct = EmptyStruct;

  println!("Initial values:");
  println!("{:?}", ts);
  println!("{:?}", os);
  println!("{:?}", es);
  println!("");

  println!("Accessing memebers: ");
  println!("(ts.0: {}, ts.1: {})", ts.0, ts.1);
  println!("os.mun: {}, os.flt: {}", os.num, os.flt);

  // pattern matching. we are 'destructuring' the structs.
  match ts {
    TupleStruct(x, y) => println!("{}, {}", x, y),
  }

  match os {
    OldStruct {num: x, flt: y} => println!("{}, {}", x, y),
  }

  match os {
    OldStruct {flt: y, ..} => { // matching part of a struct
      // no-op
    }
  }
}
