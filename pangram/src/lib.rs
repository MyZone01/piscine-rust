// Instructions
// Create a function named is_pangram which returns true if the given string is a pangram.

// A pangram is a sentence which uses every letter of the alphabet at least once.

// Example: "The quick brown fox jumps over the lazy dog."

// Expected functions
// pub fn is_pangram(s: &str) -> bool {

// }
// Usage
// Here is a program to test your function.

// use pangram::*;

// fn main() {
//     println!(
//         "{}",
//         is_pangram("the quick brown fox jumps over the lazy dog!")
//     );
//     println!("{}", is_pangram("this is not a pangram!"));
// }
// And its output:

// $ cargo run
// true
// false
// $

pub fn is_pangram(s: &str) -> bool {
    if s.len() < 26 {
        return false;
    }
    let low = s.to_lowercase();
    "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .all(|c| low.contains(c))
}
