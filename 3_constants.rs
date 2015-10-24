fn main() {
  const N: i64 = 5345444343444; // always prefer const
  static NN: i64 = 453565435;
  static mut NNN: i32 = 4443; // mutable static types are considered unsafe.
  unsafe { 
    NNN += 1;
  }
//static NAME: &'static str = "hi";
  println!("{}", N);
  println!("{}", NN);
  unsafe {
    println!("{}", NNN);
  }
}
