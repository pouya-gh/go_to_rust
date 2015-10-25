use std::collections::HashMap;

fn main() {
  let mut a_map: HashMap<&str, i32> = HashMap::new();
  let mut a_map2 = HashMap::<i32, &str>::new();

  a_map.insert("one", 1);
  a_map.insert("two", 2);
  
  for (k, v) in &a_map { // this is how you iterate over elements of a HashMap.
    println!("{}: {}", k, v);
  }

  match a_map.get("one") {  
    Some(v) => println!("printing element '{}': {}", "one", v), // v is immutable.
    None => {}
  }
  

  println!("\n___________\n");  

  a_map2.insert(1, "one");
  a_map2.insert(2, "tow");
  
  for (k, v) in &a_map2 { println!("{}: {}", k, v); }

  println!("\n___________\n");
  println!("fixed:\n");
  if let Some(x) = a_map2.get_mut(&2) { // this is how you get an mutable refrence.
    *x = "two";
  }

  // 'remove' method receives a key and delete the element.
  // it expects a refrence. more on this in 'refrences' example.
  a_map2.remove(&1); 

  for (k, v) in &a_map2 { println!("{}: {}", k, v); }
}
