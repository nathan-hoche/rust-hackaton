
fn add(a:i32, b:i32) -> i32 {
    a + b
}

fn slice_sum(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for element in slice {
        sum += element;
    }
    sum
}

fn print_modulo(number: i32, modulo: i32) {
    println!("{}", number % modulo);
}

fn main() {
    println!("{}", add(1, 2));
    println!("{}", slice_sum(&[1, 2, 3]));
    print_modulo(10, 2);
  }