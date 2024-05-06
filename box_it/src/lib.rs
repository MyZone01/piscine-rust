// Instructions
// Create the following functions:

// transform_and_save_on_heap: which accepts a string of numbers separated by spaces. If a number has a 'k' as a suffix it should be multiplied by 1000. The function transforms those numbers into a vector of u32, and saves them in the heap using Box.

// take_value_ownership: which accepts the return value from transform_and_save_on_heap, unboxes the value, and returns it.

// Expected Functions
// pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {

// }
// pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {

// }
// Usage
// Here is a program to test your functions

// use box_it::*;

// fn main() {
//     let new_str = String::from("5.5k 8.9k 32");

//     // creating a variable and we save it in the Heap
//     let a_h = transform_and_save_on_heap(new_str);
//     println!("Box value : {:?}", &a_h);
//     println!("size occupied in the stack : {:?} bytes", (std::mem::size_of_val(&a_h)));

//     let a_b_v = take_value_ownership(a_h);
//     println!("value : {:?}", &a_b_v);
//     println!("size occupied in the stack : {:?} bytes", (std::mem::size_of_val(&a_b_v)));
//     // whenever the box, in this case "a_h", goes out of scope it will be deallocated, freed
// }
// And its output:

// $ cargo run
// Box value : [5500, 8900, 32]
// size occupied in the stack : 8 bytes
// value : [5500, 8900, 32]
// size occupied in the stack : 24 bytes
// $

pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let v: Vec<u32> = s
        .split_whitespace()
        .map(|num| {
            let mut num = num.to_string();
            if num.ends_with('k') {
                num.pop();
                (num.parse::<f32>().unwrap() * 1000.0) as u32
            } else {
                num.parse::<u32>().unwrap()
            }
        })
        .collect();
    Box::new(v)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}
