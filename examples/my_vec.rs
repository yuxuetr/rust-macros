#[macro_export]
macro_rules! my_vec {
  () => { Vec::new() };
  ($elem: expr; $n:expr) => {
    std::vec::from_elem($elem, $n)
  };
  ($($x: expr), *) => {
    {
      let mut temp_vec = Vec::new();
      $(temp_vec.push($x);)*
      temp_vec
    }
  };
}

fn main() {
  let v1 = my_vec! { 1, 2, 3 };
  println!("{:?}", v1);
  let v2: Vec<i32> = my_vec![];
  println!("{:?}", v2);
  let v3 = my_vec![1; 4];
  println!("{:?}", v3);
}
