use std::collections::HashMap;

fn main() {
    let mut hash = HashMap::new();
    hash.insert("Daniel", 122);
    hash.insert("Ashley", 333);
    hash.insert("Katie", 334);
    hash.insert("Robert", 14);

    println!(
        "The smallest of the elements in the HashMap is {}",
        minor(hash)
    );
}

fn minor(hash: HashMap<&str, i32>) -> i32 {
    *hash.values().max().unwrap_or(&0)
}
