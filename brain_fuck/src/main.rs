use std::io::stdin;

const SIZE: usize = 30000;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    brain_fuck(&args[1]);
}

fn brain_fuck(code: &str) {
    let code: Vec<char> = code.chars().collect();
    let n = code.len();

    let mut memory = [0u8; SIZE];
    let mut pointer: usize = 0;
    let mut i = 0;

    while i < n {
        let current = code[i];
        match current {
            '>' => pointer += 1,
            '<' => pointer -= 1,
            '+' => memory[pointer] += 1,
            '-' => memory[pointer] -= 1,
            '.' => print!("{}", memory[pointer] as char),
            ',' => input(&pointer, &mut memory),
            '[' => {
                let mut opened_bracket = 0;
                if memory[pointer] == 0 {
                    while code[i] != ']' || opened_bracket > 1 {
                        if code[i] == '[' {
                            opened_bracket += 1;
                        } else if code[i] == ']' {
                            opened_bracket -= 1;
                        }
                        i += 1;
                    }
                }
            }
            ']' => {
                let mut opened_bracket = 0;
                if memory[pointer] != 0 {
                    while code[i] != '[' || opened_bracket > 1 {
                        if code[i] == ']' {
                            opened_bracket += 1;
                        } else if code[i] == '[' {
                            opened_bracket -= 1;
                        }
                        i -= 1;
                    }
                }
            }
            _ => (),
        }
        i += 1;
    }
}

fn input(ptr: &usize, mem: &mut [u8]) {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("No");
    let input = input.chars().next().unwrap();
    mem[*ptr] = input as u8;
}
