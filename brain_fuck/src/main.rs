use std::io::stdin;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    brain_fuck(&args[1]);
}

pub fn brain_fuck(code: &str) {
    let mut memory: [u8; 30000] = [0; 30000];
    let code: Vec<char> = code.chars().collect();
    let mut pointer: usize = 0;
    let mut i = 0;
    while i < code.len() {
        let current = code[i];
        match current {
            '>' => pointer += 1,
            '<' => pointer -= 1,
            '+' => memory[pointer] += 1,
            '-' => memory[pointer] -= 1,
            '.' => print!("{}", memory[pointer] as char),
            ',' => input(&pointer, &mut memory),
            '[' => jump_past(&pointer, &memory, &code, &mut i),
            ']' => jmp_back(&pointer, &memory, &code, &mut i),
            _ => (),
        }
        i += 1;
    }
}

fn input(ptr: &usize, mem: &mut [u8]) {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Incorrect input");

    let input = input.chars().next().unwrap();
    mem[*ptr] = input as u8;
}

fn jump_past(pointer: &usize, memory: &[u8], code: &[char], i: &mut usize) {
    if memory[*pointer] == 0 {
        let mut non_closing_braces = 0;
        for (j, &item) in code[*i + 1..].iter().enumerate() {
            if item == '[' {
                non_closing_braces += 1;
            } else if item == ']' {
                if non_closing_braces > 0 {
                    non_closing_braces -= 1;
                } else {
                    *i = *i + 1 + j;
                    break;
                }
            }
        }
    }
}

fn jmp_back(pointer: &usize, memory: &[u8], code: &[char], i: &mut usize) {
    if memory[*pointer] != 0 {
        let mut non_opening_braces = 0;
        for (j, &item) in code[..*i].iter().rev().enumerate() {
            if item == ']' {
                non_opening_braces += 1;
            } else if item == '[' {
                if non_opening_braces > 0 {
                    non_opening_braces -= 1;
                } else {
                    *i = *i - 1 - j;
                    break;
                }
            }
        }
    }
}
