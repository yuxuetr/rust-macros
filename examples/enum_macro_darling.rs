use rust_macros::EnumFromDarling;

#[derive(Debug, EnumFromDarling)]
enum Direction<T> {
  Up(DirectionUp<T>),
  Down,
  Left(u32),
}

#[derive(Debug)]
struct DirectionUp<T> {
  speed: T,
}

impl<T> DirectionUp<T> {
  fn new(speed: T) -> Self {
    Self { speed }
  }
}

fn main() {
  let up: Direction<i32> = DirectionUp::new(42).into();
  println!("{:?}", up);
}
