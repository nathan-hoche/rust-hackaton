# Exercice 1: Lifetimes

Given the following code, create a method called `make_person` that creates an instance of the structure `Person` taking a string slice as parameter.

```rust
fn main() {
    let person = Person::make_person("name");
    println!("Person name: {}", person.name)
}

struct Person<'a> {
    name: &'a str,
}

```

[Documentation](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html)