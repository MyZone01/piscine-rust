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

pub mod roman_numbers {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum RomanDigit {
        Nulla,
        I,
        V,
        X,
        L,
        C,
        D,
        M,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct RomanNumber(pub Vec<RomanDigit>);

    impl From<u32> for RomanDigit {
        fn from(n: u32) -> Self {
            match n {
                0 => RomanDigit::Nulla,
                1 => RomanDigit::I,
                5 => RomanDigit::V,
                10 => RomanDigit::X,
                50 => RomanDigit::L,
                100 => RomanDigit::C,
                500 => RomanDigit::D,
                1000 => RomanDigit::M,
                _ => RomanDigit::Nulla,
            }
        }
    }

    impl From<u32> for RomanNumber {
        fn from(n: u32) -> Self {
            let mut roman_number = RomanNumber(vec![]);
            let mut n = n;
            let divisors = [1000, 500, 100, 50, 10, 5, 1];
            for &divisor in divisors.iter() {
                let digit = n / divisor;
                n %= divisor;
                roman_number.0.push(RomanDigit::from(digit * divisor));
            }
            roman_number
        }
    }
}
