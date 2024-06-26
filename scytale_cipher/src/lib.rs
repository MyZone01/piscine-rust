// Instructions
// Create a function which creates a scytale cipher (also known as spartan cipher).

// In practice, it is represented by a strip wrapped around a cylinder. The message is written across the loops of the strip (not along the strip). The message becomes coded if the radius of the cylinder changes, or the strip is removed from the cylinder.

// Your function should recreate the scytale cipher, so that the String represents the message, and the u32 represents the number of times the strip is wrapped around the cylinder.

// Example
// size 6: "scytale Code" -> "sec yCtoadle"

// --------------------------------
//   |s|  |c|  |y|  |t|  |a|  |l|
//   |e|  | |  |C|  |o|  |d|  |e|
// --------------------------------
// size 8: "scytale Code" -> "sCcoydtea l e"

// ------------------------------------------
//   |s|  |c|  |y|  |t|  |a|  |l|  |e|  | |
//   |C|  |o|  |d|  |e|  | |  | |  | |  | |
// ------------------------------------------
// Expected Functions
// fn scytale_cipher(message: String, i: u32) -> String {
// }
// Usage
// Here is a program to test your function

// fn main() {
//     println!("\"scytale Code\" size=6 -> {:?}", scytale_cipher(String::from("scytale Code"), 6)));
//     println!("\"scytale Code\" size=8 -> {:?}", scytale_cipher(String::from("scytale Code"), 8)));
// }
// And its output

// $ cargo run
// "scytale Code" size=6 -> "sec yCtoadle"
// "scytale Code" size=8 -> "sCcoydtea l e"
// $
// (len / i).ceil() * i
// {:<width$}
// skip j step by i

pub fn scytale_cipher(message: String, i: u32) -> String {
    let padded_message = format!(
        "{:<width$}",
        message,
        width = (message.len() as f32 / i as f32).ceil() as usize * i as usize
    );
    (0..i)
        .flat_map(|j| padded_message.chars().skip(j as usize).step_by(i as usize))
        .collect::<String>()
        .trim()
        .to_string()
}
