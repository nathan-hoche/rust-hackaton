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
