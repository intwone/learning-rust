#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

// impl Rectangle {
//   fn area(&self) -> u32 {
//     self.height * self.width
//   }

//   fn double_area(&self) -> u32 {
//     (self.height * self.width) * 2
//   }

//   fn can_hold(&self, other: &Rectangle) -> bool {
//     self.height > other.height && self.width > other.width
//   }
// }

/**
 * Associate functions
 * Self é um alias para o valor que vem depois de impl, nesse caso, Rectangle.
 * Ambos impl abaixo fazem a mesma coisa
 */

impl Rectangle {
  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size,
    }
  }
}

// impl Rectangle {
//   fn square(size: u32) -> Self {
//     Self {
//       width: size,
//       height: size,
//     }
//   }
// }

fn main() {
  let rect = Rectangle {
    width: 30,
    height: 50,
  };

  let rect2 = Rectangle {
    width: 100,
    height: 200,
  };

  // println!("The area is {}", rect.area());
  // println!("The area is {:?}", rect.double_area());
  // println!("Can hold {:?}", rect.can_hold(&rect2));
  println!("Can hold {:?}", Rectangle::square(10));
}
