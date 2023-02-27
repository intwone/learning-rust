#[derive(Debug)]
enum IpAddrKind {
  V4(String),
  V6(String),
}

#[derive(Debug)]
enum IpAddrKind2 {
  V4,
  V6,
}

#[derive(Debug)]

struct IpAddr {
  kind: IpAddrKind,
  address: String,
}

struct IpAddr2 {
  kind: IpAddrKind2,
  address: String,
}

enum IpAddr3 {
  V4(u8, u8, u8, u8),
  V6(String),
}

struct Ipv4Addr {}

struct Ipv6Addr {}

enum IpAddr4 {
  V4(Ipv4Addr),
  V6(Ipv6Addr),
}

#[derive(Debug)]
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

impl Message {
  fn call(&self, nu: i32) {
    println!("{:?}", self);
    println!("{:?}", nu);
  }
}

fn main() {
  let four = IpAddrKind2::V4;
  let six = IpAddrKind2::V6;

  println!("{:?}", four);
  println!("{:?}", six);

  route(IpAddrKind::V4(String::from("127.0.0.1")));

  let home = IpAddr2 {
    kind: IpAddrKind2::V4,
    address: String::from("127.0.0.1"),
  };

  let loopback = IpAddr2 {
    kind: IpAddrKind2::V6,
    address: String::from("::1"),
  };

  let home2 = IpAddr3::V4(127, 1, 1, 0);
  let loopback = IpAddr3::V6(String::from("::1"));

  let message = Message::Write(String::from("hello"));
  let move_message = Message::Move { x: 40, y: 50 };
  let quit = Message::Quit;
  message.call(10);
  move_message.call(20);
  quit.call(20);
}

fn route(ip_kind: IpAddrKind) {}
