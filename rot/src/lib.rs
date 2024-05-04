// Instructions
// In this exercise, if you do not know about it already, you will learn about the rotational cipher "ROT13".

// A ROT13 applied the Latin alphabet:

// - Plain:  abcdefghijklmnopqrstuvwxyz
//           ||||||||||||||||||||||||||
// - Cipher: nopqrstuvwxyzabcdefghijklm
// You will create a similar rotate function that is a better version of the ROT13 cipher.

// Your function will receive a String and an i8, and will rotate each letter of that string by the number of times described by the second argument. Positive numbers rotate to the right, negative numbers rotate to the left.

// Only letters should be rotated. Numbers and punctuation should be left unchanged.

// Expected functions
// pub fn rotate(input: &str, key: i8) -> String {

// }
// Usage
// Here is a program to test your function.

// use rot::*;

// fn main() {

//     println!("The letter \"a\" becomes: {}", rotate("a", 26));
//     println!("The letter \"m\" becomes: {}", rotate("m", 0));
//     println!("The letter \"m\" becomes: {}", rotate("m", 13));
//     println!("The letter \"a\" becomes: {}", rotate("a", 15));
//     println!("The word \"MISS\" becomes: {}", rotate("MISS", 5));
//     println!(
//         "The decoded message is: {}",
//         rotate("Gur svir obkvat jvmneqf whzc dhvpxyl.", 13)
//     );
//     println!(
//         "The decoded message is: {}",
//         rotate("Mtb vznhpqd ifky ozrunsl ejgwfx ajc", 5)
//     );
//     println!(
//         "Your cypher wil be: {}",
//         rotate("Testing with numbers 1 2 3", 4)
//     );
//     println!("Your cypher wil be: {}", rotate("Testing", -14));
//     println!("The letter \"a\" becomes: {}", rotate("a", -1));
// }
// And its output:

// $ cargo run
// The letter "a" becomes: a
// The letter "m" becomes: m
// The letter "m" becomes: z
// The letter "a" becomes: p
// The word "MISS" becomes: RNXX
// The decoded message is: The five boxing wizards jump quickly.
// The decoded message is: Ryg aesmuvi nkpd tewzsxq jolbkc foh
// Your cypher wil be: Xiwxmrk amxl ryqfivw 1 2 3
// Your cypher wil be: Fqefuzs
// The letter "a" becomes: z
// $

pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                let base = if c.is_lowercase() { b'a' } else { b'A' } as i8;
                (((c as i8 - base + key) % 26 + 26) % 26 + base) as u8 as char
            } else {
                c
            }
        })
        .collect()
}
