use std::io;

fn main() {
  let mut input = String::new();

  let mut s1 = &mut input;
  let mut s2 = &mut input;
  // second mutable borrow restriction prevent data race
  println!("{} {}", s1, s2);

  some_fn(&mut input);
  io::stdin().read_line(&mut input);
  let mut mars_weight = calculate_weight_on_mars(100.0);
  println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
  (weight / 9.81) * 3.711
}

fn some_fn(s: &mut String) {
  s.push_str("a");
}
