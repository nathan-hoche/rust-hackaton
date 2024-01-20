fn main() {
    let name = "Thomas";
    let mut age = 19;
    let max_age = 100;

    age = age + 1;
    println!("Hello, {}!", name);
    println!(
        "You are {} years old and you can still live {} years.",
        age,
        max_age - age
    );
  }