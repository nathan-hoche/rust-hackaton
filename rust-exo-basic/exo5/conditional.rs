fn evaluate_number(number: i32) {
    if number > 0 {
        println!("Positive");
    } else if number < 0 {
        println!("Negative");
    } else {
        println!("Zero");
    }
}

fn main() {
  evaluate_number(0);
  evaluate_number(5);
  evaluate_number(-5);
}