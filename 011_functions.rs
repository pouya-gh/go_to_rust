fn say_this(arg: &String) { // no return type. it actually returns empty tuple. '()'
//               ^ what is this? check out the example about references.
    println!("{}", arg);
}

fn return_something() -> String {
  // return "Something".to_string();
  "Something".to_string() // pay attention that there is no ';' here. if we write a semicolon, the functions tries to return an empty tuple.
}

fn main() {
  let tmp: String = return_something();
  say_this(&tmp); 
}