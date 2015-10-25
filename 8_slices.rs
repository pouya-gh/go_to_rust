fn main() {
  let mut a = [11i64, 22, 33, 44, 55, 66];
  let b: &mut [i64] = &mut a[1..4]; // this is how you turn an array into a slice. 
  for ii in b { println!("{}", ii) } 
}
