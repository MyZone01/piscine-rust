// ## prev_prime

// ### Instructions

// Create a **function** which returns the first prime number which is less than or equal to the `u64` passed as an argument.

// If there are no smaller primes, the function should return `0`.

// ### Expected function

// ```rust
// pub fn prev_prime(nbr: u64) -> u64  {

// }
// ```

// ### Usage

// Here is a possible program to test your function :

// ```rust
// use previousprime::*;

// fn main() {
//     println!("The previous prime number before 34 is: {}", prev_prime(34));
// }
// ```

// And its output :

// ```console
// $ cargo run
// The previous prime number before 34 is: 31
// $
// ```

// pub fn prev_prime(nbr: u64) -> u64 {
//     if nbr < 2 {
//         return 0;
//     }
//     let mut i = nbr - 1;
//     while i > 1 {
//         if is_prime(i) {
//             return i;
//         }
//         i -= 1;
//     }
//     0
// }

// fn is_prime(num: u64) -> bool {
//     if num <= 1 {
//         return false;
//     }

//     for i in 2..=(num as f64).sqrt() as u64 {
//         if num % i == 0 {
//             return false;
//         }
//     }

//     true
// }

pub fn prev_prime(nbr: u64) -> u64 {
    (2..nbr).rev().find(|&i| is_prime(i)).unwrap_or(0)
}

pub fn is_prime(num: u64) -> bool {
    (2..=(num as f64).sqrt() as u64).all(|i| num % i != 0)
}
