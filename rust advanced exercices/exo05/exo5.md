# Exercice 5: Macros

Given the following code, create a macro named log! that takes two arguments: a string and a message. The string can be either "info", "warning" or "error". The macro should print the message with the corresponding prefix.

```rust
fn main() {
    log!("info", "This is an informational message.");
    log!("warning", "This is a warning message.");
    log!("error", "This is an error message.");
}
```

- Macro [documentation](https://doc.rust-lang.org/book/ch19-06-macros.html)
