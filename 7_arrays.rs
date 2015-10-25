fn main() {
  let mut a: [i32; 4] = [11,22,33,44];
  //             ^ an array of 4 32bit integers.
  for ii in &mut a[..] { *ii = *ii + 1 }
  //                ^ this is a 'range'. more on this later.
  for i in &a[..] { // this is how you iterate over an array.
  //        ^ this is a 'slice'. more on this later.
    println!("{}", i);
  } 
}
