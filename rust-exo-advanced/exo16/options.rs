
fn is_none(option: Option<i32>) -> bool {
    option.is_none()
}

fn get_str(vec: &[String], str: &str) -> Option<String> {
    let mut has_str = false;
    for s in vec {
        if s == str {
            has_str = true;
        }
    }
    if has_str {
        Some(str.to_string())
    } else {
        None
    }
}

fn main() {
    println!("{}", is_none(Some(1)));
    println!("{}", is_none(None));

    println!("{:?}", get_str(&["Hello".to_string(), "World".to_string()], "World"));
}