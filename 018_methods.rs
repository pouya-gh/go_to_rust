#[derive(Debug)]
struct Rectangle {
  x: i32,
  y: i32,
}

// you can define methods for a struct with 'impl' keyword.
// first argument of methods must be self. there are 3 types of self 
// as you will see.
// there is also 'static functions' or as we call them in rust 'associated functions'
// 'associated functions' don't have a 'self' argument.
impl Rectangle {
  fn new(x: i32, y: i32) -> Rectangle {
    Rectangle{x: x, y: y}
  }

  fn area(&self) -> i32 { // self is an immuable reference.
    self.x * self.y
  }

  fn set_x(&mut self, x: i32) { // self is a mutable reference.
    self.x = x;
  }

  fn consume_self(self) { // this method take ownership.
    println!("goodbye!");
  }
}

fn main() {
  let mut rect = Rectangle::new(4, 7); // this how we call an associated function.
  println!("{:?}", rect);
  println!("the area is: {}", rect.area()); // this how we call a method.
  println!("setting x ...");
  rect.set_x(10);
  println!("{:?}", rect);
  rect.consume_self(); // you can no longer call methods on 'rect'.
}
