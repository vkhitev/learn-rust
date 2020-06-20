fn area(width: u32, height: u32) -> u32 {
  width * height
}

fn area_t(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn area_s(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

impl Rectangle {
  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size,
    }
  }
}

fn main() {
  let width1 = 30;
  let height1 = 50;

  let rectangle = Rectangle {
    width: width1,
    height: height1,
  };

  let _area1 = area(width1, height1);
  let _area2 = area_t((width1, height1));
  let _area3 = area_s(&rectangle);

  let square = Rectangle::square(30);
  let square2 = Rectangle::square(20);

  println!("Rectangle: {:#?}", rectangle);
  println!("Area: {}", rectangle.area());
  println!("Can hold: {}", square.can_hold(&square2));
}
