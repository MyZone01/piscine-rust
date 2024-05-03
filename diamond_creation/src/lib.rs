// Instructions
// Build the function make_diamond which takes a letter as an input, and returns a diamond.

// Rules:

// The first and last row contain one 'A'.
// The given letter has to be at the widest point.
// All rows, except the first and last, have exactly two identical letters.
// All rows have as many trailing spaces as leading spaces. This may be 0.
// The diamond is vertically symmetrical, and horizontally symmetrical.
// The width of the diamond equals its height.
// The top half has letters in ascending order (abcd).
// The bottom half has letters in descending order (dcba).
// Expected functions
// pub fn get_diamond(c: char) -> Vec<String> {

// }
// Usage
// Here is a program to test your function.

// use diamond_creation::*;

// fn main() {
//     println!("{:?}", get_diamond('A'));
//     println!("{:?}", get_diamond('C'));
//     for line in get_diamond('C') {
//         println!("{}", line);
//     }
// }
// And its output:

// $ cargo run
// ["A"]
// ["  A  ", " B B ", "C   C", " B B ", "  A  "]
//   A
//  B B
// C   C
//  B B
//   A
// $

pub fn get_diamond(c: char) -> Vec<String> {
    let mut result = Vec::new();
    let n = (c as u8 - b'A' + 1) as usize;
    for i in 0..n {
        let spaces = " ".repeat(n - i - 1);
        let mut s = format!("{}{}", spaces, (b'A' + i as u8) as char);
        if i > 0 {
            let inner_spaces = " ".repeat(2 * i - 1);
            s = format!("{}{}{}", s, inner_spaces, (b'A' + i as u8) as char);
        }
        s = format!("{}{}", s, spaces);
        result.push(s);
    }
    for i in (0..n - 1).rev() {
        let spaces = " ".repeat(n - i - 1);
        let mut s = format!("{}{}", spaces, (b'A' + i as u8) as char);
        if i > 0 {
            let inner_spaces = " ".repeat(2 * i - 1);
            s = format!("{}{}{}", s, inner_spaces, (b'A' + i as u8) as char);
        }
        s = format!("{}{}", s, spaces);
        result.push(s);
    }
    result
}
