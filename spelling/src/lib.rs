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
        let mut chunk = n % 1000;
        n /= 1000;
        let mut chunk_str = String::new();

        if chunk >= 100 {
            chunk_str.push_str(units[(chunk / 100) as usize]);
            chunk_str.push_str(" ");
            chunk_str.push_str(hundreds[1]);
            chunk %= 100;
            if chunk > 0 {
                chunk_str.push_str(" ");
            }
        }

        if chunk >= 11 && chunk <= 19 {
            chunk_str.push_str(teens[(chunk - 10) as usize]);
        } else if chunk >= 20 {
            chunk_str.push_str(tens[(chunk / 10) as usize]);
            chunk %= 10;
            if chunk > 0 {
                chunk_str.push_str("-");
            }
        }

        if chunk > 0 && chunk < 10 {
            chunk_str.push_str(units[chunk as usize]);
        }

        if i > 0 && chunk > 0 {
            chunk_str.push_str(" ");
            chunk_str.push_str(thousands[i]);
        }

        if !result.is_empty() && !chunk_str.is_empty() {
            result = format!("{} {}", chunk_str, result);
        } else {
            result = format!("{}{}", chunk_str, result);
        }

        i += 1;
    }

    result.trim().trim_matches('-').to_string()
}
