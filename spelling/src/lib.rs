// Instructions
// In this exercise, you'll create the function spell that will spell a generated number.

// Here are some examples of what your function should return:

// 1 -> "one"
// 14 -> "fourteen".
// 96 -> "ninety-six"
// 100 -> "one hundred".
// 101 -> "one hundred one"
// 348 -> "three hundred forty-eight"
// 1002 -> "one thousand two".
// 1000000 -> "one million"
// Only positive numbers will be tested, up to "one million".

// Expected function
// pub fn spell(n: u64) -> String {

// }
// Usage
// Here is a program to test your function.

// use spelling::*;

// fn main() {
//     println!("{}", spell(348));
//     println!("{}", spell(9996));
// }
// And its output:

// $ cargo run
// three hundred forty-eight
// nine thousand nine hundred ninety-six
// $

pub fn spell(n: u64) -> String {
    let mut result = String::new();
    let mut n = n;
    let mut i = 0;
    let units = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let teens = [
        "",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let tens = [
        "", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
    let thousands = ["", "thousand", "million"];
    let hundreds = ["", "hundred"];

    if n == 0 {
        return "zero".to_string();
    }

    while n > 0 {
        let mut current = n % 1000;
        n /= 1000;
        let mut current_str = String::new();

        if current >= 100 {
            current_str.push_str(units[(current / 100) as usize]);
            current_str.push(' ');
            current_str.push_str(hundreds[1]);
            current_str.push(' ');
            current %= 100;
        }

        if current >= 20 {
            current_str.push_str(tens[(current / 10) as usize]);
            current_str.push('-');
            current %= 10;
        }

        if current >= 10 {
            current_str.push_str(teens[(current % 10) as usize]);
            current_str.push(' ');
            current = 0;
        }

        if current > 0 {
            current_str.push_str(units[current as usize]);
            current_str.push(' ');
        }

        if !current_str.is_empty() {
            current_str.push_str(thousands[i]);
            current_str.push(' ');
        }

        result = current_str + &result;
        i += 1;
    }

    if result.ends_with('-') {
        result.pop();
    }

    result.trim().to_string()
}
