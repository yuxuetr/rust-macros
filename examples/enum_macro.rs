use rust_macros::EnumFrom;

#[derive(EnumFrom, Debug)]
enum Direction<T> {
  Up(DirectionUp<T>),
  Down,
}

#[derive(Debug)]
struct DirectionUp<T> {
  speed: T,
}

fn main() {
  let up: Direction<i32> = DirectionUp::new(42).into();
  println!("{:?}", up);
}

impl<T> DirectionUp<T> {
  fn new(speed: T) -> Self {
    Self { speed }
  }
}
