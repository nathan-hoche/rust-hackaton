fn print_slices_element(slice: &[String]) {
  for element in slice {
    println!("{}", element);
  }
}

fn main() {
  print_slices_element(&["Thomas".to_string(), "Nassim".to_string(), "Guillaume".to_string()]);
}