use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input);
  println!("input: {}", input);
  let mut mars_weight = calculate_weight_on_mars(100.0);
  println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
  (weight / 9.81) * 3.711
}
