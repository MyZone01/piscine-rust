// Vector is
pub fn insert(vec: &mut Vec<String>, item: String) {
    vec.push(item);
}

pub fn at_index(vec: &Vec<String>, index: usize) -> String {
    // let item = vec.get(index);
    // match item {
    //     Some(value) => value.to_string(),
    //     None => String::from("Index out of bounds"),
    // }
    vec.get(index)
        .unwrap_or(&String::from("default"))
        .to_string()
}
