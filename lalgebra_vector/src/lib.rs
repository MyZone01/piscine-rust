// Instructions
// A vector in linear algebra is defined as "anything that can be added, and that can be multiplied by a scalar".

// Define the associated function dot, that calculates the dot product between two vectors. If the vectors are of different lengths, return None.

// Note: Vector must implement Debug, Clone, Eq and PartialEq.

// Expected Functions and Structure
// pub struct Vector<T: Scalar>(pub Vec<T>);

// use std::ops::Add;

// impl Add for Vector<T> {
// }

// impl Vector<T> {
// 	pub fn new() -> Self {
// 	}

// 	pub fn dot(&self, other: &Self) -> Option<T> {
// 	}
// }
// Usage
// Here is a program to test your function.

// use lalgebra_vector::*;

// fn main() {
// 	let vector_1: Vector<i64> = Vector(vec![1, 3, -5]);
// 	let vector_2: Vector<i64> = Vector(vec![4, -2, -1]);
// 	println!("{:?}", vector_1.dot(&vector_2));
// 	println!("{:?}", vector_1 + vector_2);
// }
// And its output:

// $ cargo run
// Some(3)
// Some(Vector([5, 1, -6]))
// $

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

use std::ops::Add;

pub trait Scalar: Add<Output = Self> + Clone {}

impl<T> Scalar for T where T: Add<Output = Self> + Clone {}

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Vector<T>>;

    fn add(self, other: Vector<T>) -> Option<Vector<T>> {
        if self.0.len() != other.0.len() {
            None
        } else {
            let sum: Vec<T> = self
                .0
                .iter()
                .zip(other.0.iter())
                .map(|(&ref a, &ref b)| a.clone() + b.clone())
                .collect();
            Some(Vector(sum))
        }
    }
}

impl<T: Scalar + std::ops::Mul + std::iter::Sum<<T as std::ops::Mul>::Output>> Vector<T> {
    pub fn new(v: Vec<T>) -> Self {
        Vector(v)
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            None
        } else {
            let dot_product: T = self
                .0
                .iter()
                .zip(other.0.iter())
                .map(|(&ref a, &ref b)| a.clone() * b.clone())
                .sum();
            Some(dot_product)
        }
    }
}
