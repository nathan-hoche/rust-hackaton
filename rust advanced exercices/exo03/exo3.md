# Exercice 3: Advanced error handling using this_error

Given the following code, fill the enum `CustomError` using `this_error` crate in order to make it compile.

```rust
#[derive(Debug)]
enum CustomError {}

fn main() -> Result<(), CustomError> {
    let input = std::fs::read_to_string("input.txt")?;
    let input = input.split(" ");

    let mut sum = 0;
    for number in input {
        let number = number.parse::<i32>()?;
        sum += number;
    }

    println!("sum: {}", sum);
    Ok(())
}

```

Crate this_error [documentation](https://docs.rs/thiserror/latest/thiserror/)
