struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  println!("The area is {}", calculate_area(30, 50));

  let rect1 = (30, 50);
  println!("The area is {}", calculare_area_typle_struct(rect1));

  let rect2 = Rectangle {
    width: 30,
    height: 50,
  };

  println!("The area is {}", calculare_area_struct(&rect2));
  println!("The area is {}", calculare_area_struct(&rect2));
}

fn calculate_area(width: u32, height: u32) -> u32 {
  width * height
}

fn calculare_area_typle_struct(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}

fn calculare_area_struct(reactangle: &Rectangle) -> u32 {
  reactangle.height * reactangle.width
}
