struct Fibonacci {
    curr: u32,
    next: u32,
}

fn main() {
  let mut fibonacci = Fibonacci { curr: 0, next: 1 };

  for i in 0..4 {
        println!("{}", fibonacci.next);
        fibonacci.curr = fibonacci.next;
        fibonacci.next = i + fibonacci.curr;
    }
}