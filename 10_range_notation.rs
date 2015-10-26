// you have already seen range notations. check arrays and slices.
// here are a few more examples.

fn main() {
  let an_array: [i32; 6] = [11, 22, 33, 44, 55, 66];

  // lower bound of a range is inclusive.
  // upper bound of a range in exclusive.
  println!("printing numbers from 1 to 2.");
  for ii in 1..3 { print!("{}, ", ii); }
  println!("");

  // range notations are used to make slices.

  // a range notation without an upper bound means 'to the end'.
  println!("printing elements from the \"third\" element to the end (index starts from 0).");
  for elem in &an_array[2..] { print!("{}, ", elem); }
  println!("");

  // a range without a lower bound means 'from the beginning'.
  println!("printing elements form the beginning to the \"second\" element.");
  for elem in &an_array[..2] { print!("{}, ", elem); }
  println!("");

  // a range without any bounds means 'from the beginning to the end'
  println!("printing elements of an array.");
  for elem in &an_array[..]  { print!("{}, ", elem); }
  println!("");

  // you can NOT do this.
  // for ii in .. { 
  //   if ii == 5 {
  //     break;
  //   }
  // }
}
