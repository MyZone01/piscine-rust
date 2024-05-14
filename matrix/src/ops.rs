// Instructions
// In this exercise, you will define some basic matrix operations, Implement traits for Add and Sub

// Remember that two matrices can only be added or subtracted if they have the same dimensions. Therefore, you must handle the possibility of failure by returning an Option<T>.

// You will be reusing your Matrix and Scalar structures defined in the matrix and lalgebra_scalar exercises.

// Expected Function
// use crate::{Matrix, Scalar};
// use std::ops::{ Add, Sub };

// impl Add for Matrix {

// }

// impl Sub for Matrix {

// }
// Usage
// Here is a program to test your function

// use matrix_ops::*;

// fn main() {
// 	let matrix = Matrix(vec![vec![8, 1], vec![9, 1]]);
// 	let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
// 	println!("{:?}", matrix + matrix_2);

// 	let matrix = Matrix(vec![vec![1, 3], vec![2, 5]]);
// 	let matrix_2 = Matrix(vec![vec![3, 1], vec![1, 1]]);
// 	println!("{:?}", matrix - matrix_2);

// 	let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
// 	let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
// 	println!("{:?}", matrix - matrix_2);

// 	let matrix = Matrix(vec![vec![1, 3], vec![9, 1]]);
// 	let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
// 	println!("{:?}", matrix + matrix_2);
// }
// And its output

// $ cargo run
// Some(Matrix([[9, 2], [10, 2]]))
// Some(Matrix([[-2, 2], [1, 4]]))
// None
// None
// $

#[derive(Debug, PartialEq)]
pub struct Matrix(pub Vec<Vec<i32>>);

use std::ops::{Add, Sub};

impl Add for Matrix {
    type Output = Option<Matrix>;

    fn add(self, other: Matrix) -> Self::Output {
        let mut result = vec![];
        if self.0.len() != other.0.len() {
            return None;
        }
        for i in 0..self.0.len() {
            if self.0[i].len() != other.0[i].len() {
                return None;
            }
            let mut row = vec![];
            for j in 0..self.0[i].len() {
                row.push(self.0[i][j] + other.0[i][j]);
            }
            result.push(row);
        }
        Some(Matrix(result))
    }
}

impl Sub for Matrix {
    type Output = Option<Matrix>;

    fn sub(self, other: Matrix) -> Self::Output {
        let mut result = vec![];
        if self.0.len() != other.0.len() {
            return None;
        }
        for i in 0..self.0.len() {
            if self.0[i].len() != other.0[i].len() {
                return None;
            }
            let mut row = vec![];
            for j in 0..self.0[i].len() {
                row.push(self.0[i][j] - other.0[i][j]);
            }
            result.push(row);
        }
        Some(Matrix(result))
    }
}
