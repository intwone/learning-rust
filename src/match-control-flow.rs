enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}

enum Coin2 {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
  Alabama,
  Alaska,
  // --snip--
}

fn value_in_cents2(coin: Coin2) -> u8 {
  match coin {
    Coin2::Penny => 1,
    Coin2::Nickel => 5,
    Coin2::Dime => 10,
    Coin2::Quarter(state) => {
      println!("State quarter from {:?}!", state);
      25
    }
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

fn add_fancy_hat() {
  println!("added");
}

fn remove_fancy_hat() {
  println!("removed");
}

fn move_player(num_spaces: u8) {
  println!("{:?}", num_spaces);
}

fn main() {
  // println!("{:?}", value_in_cents(Coin::Quarter));
  // println!("{:?}", value_in_cents2(Coin2::Quarter(UsState::Alaska)));
  // println!("{:?}", value_in_cents2(Coin2::Penny));

  let five = Some(4);
  let six = plus_one(five);
  let none = plus_one(None);

  println!("{:?}", plus_one(five));
  println!("{:?}", plus_one(six));
  println!("{:?}", plus_one(none));

  let dice_roll = 7;

  match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
  }
}
