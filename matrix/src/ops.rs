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
