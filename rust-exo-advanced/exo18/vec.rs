
fn remove_first_and_last(vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec = vec.clone();
    new_vec.remove(0);
    new_vec.pop();
    new_vec
}

fn concat_vec(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {
    let mut new_vec = vec1.clone();
    new_vec.extend(vec2);
    new_vec.iter().map(|x| x*2).collect()
}

fn main() {
    println!("{:?}", remove_first_and_last(vec![1, 2, 3, 4, 5]));
    println!("{:?}", concat_vec(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]));
  }