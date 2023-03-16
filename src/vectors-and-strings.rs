fn main() {
  // let config_max = Some(3u8);
  // match config_max {
  //   Some(max) => println!("The maximum is configured to be {}", max),
  //   _ => (),
  // }

  let config_max = Some(-1.2);
  if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
  } else {
    println!("Faz")
  }

  let mut v = Vec::new();
  v.push(10);
  v.push(10);
  v.push(10);

  let vector = vec!["Cassio", "Elanne", "Catia"];

  let third = v[2];

  match third {
    10 => println!("here"),
    _ => println!("there"),
  }

  let forth = v.get(2);

  let err = v.get(100);

  match err {
    None => println!("number out of the range"),
    _ => (),
  }

  match forth {
    Some(forth) => println!("The number is {:?}", forth),
    None => println!("Nothing here"),
  }

  println!("{:?}", &v);
  println!("{:?}", vector);
  println!("{:?}", third);

  for number in &v {
    println!("{:?}", number);
  }

  for number in &mut v {
    *number = *number + 50;
  }

  println!("{:?}", v);

  let nome = "cassio oliveira";

  println!("{nome}");

  let mut s1 = String::from("foo");
  let s2 = "bar";
  s1.push_str(s2);
  s1.push_str("l");
  s1.push('W');
  println!("s2 is {s2}");
  println!("s1 is {s1}");
}
