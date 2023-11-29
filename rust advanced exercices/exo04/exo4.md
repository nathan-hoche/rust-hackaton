# Exercice 4: Multithreading

Given the following code, fill the method chunked_vec and multi_threaded_sum in order to make it compile and run.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let v: Vec<i32> = (0..100_000).collect();

    println!("The sum is: {}", multi_threaded_sum(v, 10));
}

// Function to split a vector into chunks
fn chunked_vec(v: &[i32], chunk_size: usize) -> Vec<Vec<i32>> {}

// Function to perform multi-threaded sum
fn multi_threaded_sum(v: Vec<i32>, thread_count: usize) -> i128 {
    let chunks = Arc::new(Mutex::new(chunked_vec(&v, v.len() / thread_count)));
    let mut handles = vec![];

    for _ in 0..thread_count {
        let chunks_clone = Arc::clone(&chunks);

        // Spawn a new thread, pop the last vector from the chunks vector and compute the sum
        let handle = thread::spawn(move || {});

        handles.push(handle);
    }

    let mut total_sum: i128 = 0;
    for handle in handles {
        // adds the sum of every handle inside total_sum
    }

    total_sum
}
```

- Arc [documentation](https://doc.rust-lang.org/std/sync/struct.Arc.html)
- Mutex [documentation](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
- Threads [documentation](https://doc.rust-lang.org/book/ch16-01-threads.html)
