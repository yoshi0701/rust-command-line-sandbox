use std::io;

fn main() {
  println!("Enter your weight (kg): ");
  let mut input = String::new();

  // add unwrap if result is error terminate excution
  io::stdin().read_line(&mut input).unwrap();

  // prevent white space
  let weight = input.trim();
  dbg!(weight);

  println!("input: {}", input);
  let mut mars_weight = calculate_weight_on_mars(100.0);
  println!("Weight on Mars: {}kg", mars_weight);
}



fn calculate_weight_on_mars(weight: f32) -> f32 {
  (weight / 9.81) * 3.711
}
