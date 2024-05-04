// ## nextprime

// ### Instructions

// Create a **function** which returns the first prime number which is greater than or equal to the `u64` passed as an argument.

// The function must be optimized, so as to avoid time-outs.

// > We consider that only positive numbers can be prime numbers.

// ### Expected function

// ```rust
// pub fn next_prime(nbr: u64) -> u64 {

// }
// ```

// ### Usage

// Here is a possible program to test your function :

// ```rust
// use nextprime::*;

// fn main() {
//     println!("The next prime after 4 is: {}", next_prime(4));
//     println!("The next prime after 11 is: {}", next_prime(11));
// }

// ```

// And its output :

// ```console
// $ cargo run
// The next prime after 4 is: 5
// The next prime after 11 is: 11
// $
// ```

// pub fn next_prime(nbr: u64) -> u64 {
//     if nbr < 2 {
//         return 2;
//     }
//     let mut i = nbr;
//     while i > 1 {
//         if is_prime(i) {
//             return i;
//         }
//         i += 1;
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

pub fn next_prime(nbr: u64) -> u64 {
    if nbr < 2 {
        return 2;
    }
    let mut i = nbr;
    while !is_prime(i) {
        i += 1;
    }
    i
}

pub fn is_prime(num: u64) -> bool {
    (2..=(num as f64).sqrt() as u64).all(|i| num % i != 0)
}
