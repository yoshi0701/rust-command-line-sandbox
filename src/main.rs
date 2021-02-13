use std::io;

fn main() {
  let mut input = String::new();
  // when reached some_fn ownership is moved, then input is no longer available
  some_fn(input);
  io::stdin().read_line(&mut input);
  let mut mars_weight = calculate_weight_on_mars(100.0);
  println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
  (weight / 9.81) * 3.711
}

fn some_fn(s: String) {}

// 1. each value in Rust is owner by a variable

// 2. when the owner goes out of scope, the value will be deallocated

// 3. there can only bt one owner at a time
