// Import the standard io library
use std::io;

fn main() {
    // Define a variable to store the riddle text (devinette)
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";

    // Define a variable to store the answer text
    let answer = "The letter e";

    // Define a variable to store the number fo tries
    let mut tries = 0;

    loop {
        // Increment the number of try
        tries += 1;

        // Print the riddle
        println!("{}", riddle);

        // Define a string to store the input user answer
        let mut user_input = String::new();

        // Wait for user input
        io::stdin().read_line(&mut user_input).unwrap();
        if user_input.trim() == answer {
            println!("Number of trials: {}", tries);
            break;
        }
    }
}
