

fn get_element_at<T>(tab: &[T], index: usize) -> Result<&T, String> {
    if index >= tab.len() {
        return Err("Index out of bounds".to_string());
    }
    Ok(&tab[index])
}

fn main() {
    println!("{:?}", get_element_at(&["Hello".to_string(), "World".to_string()], 3));
    println!("{:?}", get_element_at(&["Hello".to_string(), "World".to_string()], 1));
}