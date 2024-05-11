// ## matrix_display

// ### Instructions

// Complete the `Matrix` struct below.

// You will need to create the `new` associated function which initializes the struct.

// You will also need to implement the `std::fmt::Display` trait, so that it prints like the example in the usage.

// ### Expected Functions and Struct

// ```rust
// pub struct Matrix(pub Vec<Vec<i32>>);

// impl Matrix {
//     pub fn new(slice: &[&[i32]]) -> Self {

//     }
// }

// use std::fmt;

// impl fmt::Display for Matrix {

// }
// ```

// ### Usage

// Here is a possible program to test your function

// ```rust
// use matrix_display::*;

// fn main() {
//     let matrix = Matrix::new(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]);
//     println!("{}", matrix);
// }

// ```

// And it's output:

// ```console
// $ cargo run
// (1 2 3)
// (4 5 6)
// (7 8 9)
// $
// ```

#[derive(Debug, Clone)]
pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        let mut matrix = Vec::new();
        for row in slice {
            matrix.push(row.to_vec());
        }
        Matrix(matrix)
    }
}

use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.0 {
            write!(f, "(")?;
            for (i, &cell) in row.iter().enumerate() {
                write!(f, "{}", cell)?;
                if i < row.len() - 1 {
                    write!(f, " ")?;
                }
            }
            writeln!(f, ")")?;
        }
        Ok(())
    }
}
