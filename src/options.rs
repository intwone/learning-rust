// enum Option<T> {
//   None,
//   Some(T),
// }

fn main() {
  let some_number = Some(5);
  let some_char = Some('c');
  let absent_number: Option<i32> = None;
  println!("{:?}", some_number);
  println!("{:?}", some_char);
  println!("{:?}", absent_number);
}
