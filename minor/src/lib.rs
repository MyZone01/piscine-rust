use std::collections::HashMap;

pub fn minor(h: HashMap<&str, i32>) -> i32 {
    let min = 0;
    for num in h.iter() {
        println!("{}, {}", num.0, num.1)
    }
    min
}
