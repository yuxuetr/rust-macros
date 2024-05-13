use rust_macros::EnumFrom;

#[derive(EnumFrom, Debug)]
enum Direction {
  Up(DirectionUp),
  Down,
}

#[derive(Debug)]
struct DirectionUp {
  speed: u32,
}

fn main() {
  let up: Direction = DirectionUp::new(42).into();
  println!("{:?}", up);
}

impl DirectionUp {
  fn new(speed: u32) -> Self {
    Self { speed }
  }
}
