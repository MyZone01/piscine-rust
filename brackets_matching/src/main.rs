// ## brackets_matching

// ### Instructions

// Create a **program** that takes an undefined number of command-line arguments. For each argument, if the expression is correctly bracketed, the program prints `OK` to the standard output followed by a newline (`'\n'`), otherwise it prints `Error` followed by a newline.

// All characters are ignored except for the following brackets:
// - parentheses `(` and `)`.
// - square brackets `[` and `]`.
// - curly braces `{` and `}`.

// Opening brackets must only be closed by the corresponding closing bracket. For example, a curly brace cannot close a square bracket.

// A `String` which does not contain any brackets is considered to be correctly bracketed.

// If there are no arguments, the program must print nothing.

// You'll need to get the command line arguments somehow, and this will get you started:

// ```rust
// fn main() {
//     let args: Vec<String> = std::env::args().collect();

//     //...
// }

// ```

// ### Usage

// ```console
// $ cargo run '(johndoe)' | cat -e
// OK
// $ cargo run '([)]' | cat -e
// Error
// $ cargo run

// $ cargo run '' '{[(0 + 0)(1 + 1)](3*(-1)){()}}' | cat -e
// OK
// OK
// ```

fn main() {
    let args: Vec<String> = std::env::args().collect();

    for arg in args.iter().skip(1) {
        if is_correctly_bracketed(arg) {
            println!("OK");
        } else {
            println!("Error");
        }
    }
}

fn is_correctly_bracketed(arg: &str) -> bool {
    let mut stack = Vec::new();

    for c in arg.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => (),
        }
    }

    stack.is_empty()
}
