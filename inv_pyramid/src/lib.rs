// ## inv_pyramid

// ### Instructions

// Create a function named `inv_pyramid` that takes a `string` as an `integer` and returns a vector of `string`s.
// This function should create a pyramid structure. Each element of the vector must be a combination of spaces and the string given

// ### Example

// i = 5

// ```console
// [
//     " >",
//     "  >>",
//     "   >>>",
//     "    >>>>",
//     "     >>>>>",
//     "    >>>>",
//     "   >>>",
//     "  >>",
//     " >"
// ]
// ```

// ### Expected Functions

// ```rust
// fn inv_pyramid(v: String, i: u32) -> Vec<String> {}
// ```

// ### Usage

// Here is a program to test your function

// ```rust
// use inv_pyramid::*;

// fn main() {
//     let a = inv_pyramid(String::from("#"), 1);
//     let b = inv_pyramid(String::from("a"), 2);
//     let c = inv_pyramid(String::from(">"), 5);
//     let d = inv_pyramid(String::from("&"), 8);

//     for v in a.iter() {
//         println!("{:?}", v);
//     }
//     for v in b.iter() {
//         println!("{:?}", v);
//     }
//     for v in c.iter() {
//         println!("{:?}", v);
//     }
//     for v in d.iter() {
//         println!("{:?}", v);
//     }
// }
// ```

// And its output

// ```console
// $ cargo run
// " #"
// " a"
// "  aa"
// " a"
// " >"
// "  >>"
// "   >>>"
// "    >>>>"
// "     >>>>>"
// "    >>>>"
// "   >>>"
// "  >>"
// " >"
// " &"
// "  &&"
// "   &&&"
// "    &&&&"
// "     &&&&&"
// "      &&&&&&"
// "       &&&&&&&"
// "        &&&&&&&&"
// "       &&&&&&&"
// "      &&&&&&"
// "     &&&&&"
// "    &&&&"
// "   &&&"
// "  &&"
// " &"
// $
// ```

pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut res = Vec::new();
    let mut spaces = i as usize;
    let mut v = v.clone();
    for _ in 0..i {
        res.push(format!("{:1$}", v, spaces));
        // use concatenation to avoid cloning
        v = v.clone() + &v;
        spaces -= 1;
    }
    spaces = 2;
    for _ in 0..i - 1 {
        res.push(format!("{:1$}", v, spaces));
        v.truncate(v.len() / 2);
        spaces += 1;
    }
    res
}
