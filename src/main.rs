use std::io;

fn main() {
  // duplicated pointer address issue (two owner)
  let mut input = String::new();
  // -> owner of valiable is moved to second pointer
  let mut s = input;
  // -> error shows :  value borrowed here after move
  io::stdin().read_line(&mut input);
  let mut mars_weight = calculate_weight_on_mars(100.0);
  println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
  (weight / 9.81) * 3.711
}

// 1. each value in Rust is owner by a variable

// 2. when the owner goes out of scope, the value will be deallocated

// 3. there can only bt one owner at a time
