use std::collections::HashMap;

fn create_hash_map(input: &[&str]) -> HashMap<String, i32> {

    let mut hash_map = HashMap::new();

    for i in input {
        let count = hash_map.entry(i.to_string()).or_insert(0);
        *count += 1;
    }

    hash_map
}

fn main() {
    println!("{:?}", create_hash_map(&["Thomas", "Thomas", "Guillaume"]));

}