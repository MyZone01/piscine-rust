// ## roman_numbers

// ### Instructions

// Implement the `From<u32>` trait to create a roman number from a number. The roman number should be in subtractive notation (the common way to write roman number I, II, III, IV, V, VI, VII, VIII, IX, X ...).

// Start by defining the digits as `RomanDigit` (`Nulla` is 0).

// Next define `RomanNumber` as a wrapper to a vector of `RomanDigit`, and implement the Trait `From<u32>`.

// ### Expected Functions and Data Structures

// ```rust
// use crate::RomanDigit::*;

// #[derive(Copy, Clone, Debug, PartialEq, Eq)]
// pub enum RomanDigit {
// 	Nulla,
// 	I,
// 	V,
// 	X,
// 	L,
// 	C,
// 	D,
// 	M,
// }

// #[derive(Debug, Clone, Debug, PartialEq, Eq)]
// pub struct RomanNumber(pub Vec<RomanDigit>);

// impl From<u32> for RomanDigit {
// }

// impl From<u32> for RomanNumber {
// }
// ```

// ### Usage

// Here is a program to test your function.

// ```rust
// use roman_numbers::RomanNumber;

// fn main() {
// 	println!("{:?}", RomanNumber::from(32));
// 	println!("{:?}", RomanNumber::from(9));
// 	println!("{:?}", RomanNumber::from(45));
// 	println!("{:?}", RomanNumber::from(0));
// }
// ```

// And its output:

// ```console
// $ cargo run
// RomanNumber([X, X, X, I, I])
// RomanNumber([I, X])
// RomanNumber([X, L, V])
// RomanNumber([Nulla])
// $
// ```
use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    IV,
    V,
    IX,
    X,
    XL,
    L,
    XC,
    C,
    CD,
    D,
    CM,
    M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(num: u32) -> Self {
        match num {
            0 => Nulla,
            1 => I,
            4 => IV,
            5 => V,
            9 => IX,
            10 => X,
            50 => L,
            90 => XC,
            100 => C,
            400 => CD,
            500 => D,
            900 => CM,
            1000 => M,
            _ => panic!("Invalid number"),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(num: u32) -> Self {
        if num == 0 {
            return RomanNumber(vec![Nulla]);
        }
        let mut num = num;
        let mut roman_number = vec![];
        let digits = [M, CM, D, CD, C, XC, L, XL, X, IX, V, IV, I];
        let roman_values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];

        for (i, &value) in roman_values.iter().enumerate() {
            while num >= value {
                num -= value;
                match digits[i] {
                    IV => {
                        roman_number.push(I);
                        roman_number.push(V);
                    }
                    IX => {
                        roman_number.push(I);
                        roman_number.push(X);
                    }
                    XL => {
                        roman_number.push(X);
                        roman_number.push(L);
                    }
                    XC => {
                        roman_number.push(X);
                        roman_number.push(C);
                    }
                    CD => {
                        roman_number.push(C);
                        roman_number.push(D);
                    }
                    CM => {
                        roman_number.push(C);
                        roman_number.push(M);
                    }
                    _ => roman_number.push(digits[i]),
                }
            }
        }

        RomanNumber(roman_number)
    }
}
