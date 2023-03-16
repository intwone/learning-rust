use std::collections::HashMap;

fn main() {
  let mut scores = HashMap::new();

  scores.insert("Blue", 10);
  scores.insert("Blue", 10000);
  scores.insert("Yellow", 20);
  scores.insert("Red", 30);

  println!("{:?}", scores);

  let team_name = "Blue";
  let score = scores.get(team_name).copied().unwrap_or(0);

  println!("{:?}", score);

  for (k, v) in scores {
    println!("{k}, {v}");
  }

  // HashMap and Ownership

  // with no copy values
  let field_name = String::from("Favorite color");
  let field_value = String::from("Blue");

  let mut map = HashMap::new();
  map.insert(&field_name, &field_value);

  println!("{:?}", field_name);

  // with copy values
  let name = "Cassio";
  let status = true;

  let mut map2 = HashMap::new();
  map2.insert(name, status);

  println!("{:?}", name);
  println!("{:?}", map2);

  let mut users = HashMap::new();
  users.insert("Cassio", 29);

  users.entry("Elanne").or_insert(50); // if has Elanne with a value keeps this value, if not, the value to be 50
  users.entry("Cassio").or_insert(10); // Cassio already have a value, then keeps this value

  println!("{:?}", &users);
  println!("{:?}", &users.entry("Elanne").or_insert(50)); // return the value of Elanne

  // update value based on the old value
  let text = "hello world wonderful world";

  let mut map = HashMap::new();

  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0); // the insert_method return an mutable of the for the value of key
    *count += 1; // here, this mutable reference (in this case is 0) is sum with 1
  }

  println!("{:?}", map);
}
