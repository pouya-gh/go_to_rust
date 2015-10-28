// a 'Trait' is what we call in C++ a 'Interface'
// a 'Trait' is a 'standard'. it says 'if a struct implemnts
// these methods, it is of a specific kind'
// 'Copy', 'Debug' and even 'Fn' are all traits. 

#[derive(Debug)]
struct Something {
  val: i32,
}

#[derive(Debug)]
struct AnothertThing {
  val: i64,
}

trait Serializable {
  fn serialize(&self) -> String;
}

trait Tst {
  fn mult_by_2(&self) -> i32;
  fn mult_by_3(&self) -> i32 { // we can have a default implementation.
    println!("implement me!");
    -1
  }
}

// we can inherite from onther traits like this: 
trait Tst2 : Tst { 
  fn mult_by_4(&self) -> i32;
}

// implementing traits is similar to implementing methods for structs.
impl Serializable for Something {
  fn serialize(&self) -> String {
    format!("{:?}", self)
  }
}

impl Serializable for AnothertThing {
  fn serialize(&self) -> String {
    format!("{:?}", self)
  }
}

// now 'Something' and 'AnothertThing' are 'Serializable'.

impl Tst for Something {
  fn mult_by_2(&self) -> i32 {
    self.val * 2
  }
}

impl Tst2 for AnothertThing {
  fn mult_by_4(&self) -> i32 {
    (self.val as i32) * 4
  }
}

impl Tst for AnothertThing { // yes! you have to implement 'Tst' for 'Tst2' seprately.
  fn mult_by_2(&self) -> i32 {
    (self.val as  i32) * 2  
  }

  fn mult_by_3(&self) -> i32 {
    (self.val as i32) * 3
  }
}

// we can make generic functions using traits.
// the syntax is as follows:
fn takes_a_tst<T: Tst + Serializable>(arg: &T) { // You have to use '+' when a generic has to implement more than one traits.
  println!("{}", arg.serialize());
  println!("multiply by 2: {}", arg.mult_by_2());
  println!("multiply by 3: {}", arg.mult_by_3());
}

fn takes_a_tst2_and_tst<T, K>(arg: &T, arg2: &K) // you can use a 'where clause' to make your code more readable.
  where T: Tst + Serializable,
        K: Tst2 + Serializable {
  println!("{}", arg.serialize());
  println!("{}", arg2.serialize());
  println!("multiply by 2: {}", arg2.mult_by_2());
  println!("multiply by 3: {}", arg2.mult_by_3());
  println!("multiply by 4: {}", arg2.mult_by_4());
}

fn main() {
  let a = Something { val: 43};
  let b = AnothertThing { val: 33 };

  takes_a_tst(&a);
  takes_a_tst2_and_tst(&a, &b);
}
